"""Development-only accounting of observed pipeline/detector facts (TASK-055)."""
from collections import Counter, defaultdict

TERMINALS=('analyzed_files','partial_files','analysis_failed_files','unsupported_files','not_applicable_files','unknown_files')
STATUSES={'Complete','Partial','Truncated','Failed','Unsupported','NotApplicable','NotReported'}


def partition(row):
    keys=('selected_files','read_ok_files','read_failed_files',*TERMINALS)
    if any(type(row.get(k)) is not int or row[k]<0 for k in keys):
        raise ValueError('missing, negative or invalid counter')
    if row['selected_files']!=row['read_ok_files']+row['read_failed_files'] or row['read_ok_files']!=sum(row[k] for k in TERMINALS):
        raise ValueError('file partition does not reconcile')


def ratio(n,d):
    return {'numerator':n,'denominator':d,'percent':100*n/d if d else None}


def rates(*,selected,enumerated,known_total,read_ok,applicable,completed):
    numbers=[selected,enumerated,read_ok,applicable,completed]+([] if known_total is None else [known_total])
    if any(type(n) is not int or n<0 for n in numbers) or not completed<=applicable<=read_ok<=selected<=enumerated or (known_total is not None and enumerated>known_total):
        raise ValueError('inconsistent measurement populations')
    enumeration=ratio(enumerated,known_total) if known_total is not None else {'numerator':enumerated,'denominator':None,'percent':None}
    return {'enumeration':enumeration,'applicability':ratio(applicable,read_ok),'completion':ratio(completed,applicable)}


def detector_counts(rows):
    seen=set();counts=defaultdict(lambda:defaultdict(Counter))
    for row in rows:
        key=(row['case'],row['detector'])
        if key in seen or row['status'] not in STATUSES:raise ValueError('duplicate or invalid detector result')
        seen.add(key);counts[row['format']][row['detector']][row['status']]+=1
    return {f:{d:dict(c) for d,c in detectors.items()} for f,detectors in counts.items()}


def collect(source, output):
    """Validate and aggregate already observed facts; never infer missing outcomes."""
    import hashlib
    import json
    from pathlib import Path
    source,output=Path(source),Path(output)
    core=json.loads((source/'core055.json').read_text())
    selection=json.loads((source/'selection055.json').read_text())
    formats=json.loads((source/'formats055.json').read_text())
    limits=json.loads((source/'limits055.json').read_text())
    rows=core['files'];seen=set()
    for row in rows:
        if row['file'] in seen:raise ValueError('duplicate file outcome')
        seen.add(row['file']);partition(row['coverage'])
        if row['coverage'][row['expected_terminal']]!=1:raise ValueError('wrong terminal outcome')
    partition(core['batch'])
    for key in ('selected_files','read_ok_files','read_failed_files',*TERMINALS):
        if core['batch'][key]!=sum(row['coverage'][key] for row in rows):raise ValueError('batch differs from per-file facts')
    cells=[];expected_detectors={'tree_sitter_detector','regex_detector','dependency_detector','certificate_detector','key_detector'}
    case_ids=set()
    for case in formats['cases']:
        if case['id'] in case_ids:raise ValueError('duplicate format case')
        case_ids.add(case['id'])
        if hashlib.sha256(bytes.fromhex(case['inputHex'])).hexdigest()!=case['inputSha256']:raise ValueError('input digest mismatch')
        if {r['detector'] for r in case['detectors']}!=expected_detectors or len(case['detectors'])!=5:raise ValueError('missing detector measurement')
        for row in case['detectors']:
            if row['case']!=case['id'] or row['format']!=case['format']:raise ValueError('detector cell belongs to another input')
            if row['status']!=row['expected']:raise ValueError('unexpected detector state')
        cells.extend(case['detectors'])
    known={name for name,_ in selection['knownInputs']}
    selected=set(selection['selected']);excluded=set(selection['enumeratedExcluded']);not_enumerated=set(selection['notEnumerated'])
    if selected & excluded or selected & not_enumerated or excluded & not_enumerated or selected|excluded|not_enumerated!=known or len(known)!=selection['knownTotal']:raise ValueError('controlled inventory does not reconcile')
    partition(selection['diagnostics']['coverage'])
    applicable=sum(row['expectedApplicable'] is True for row in rows)
    completed=sum(row['expectedApplicable'] is True and row['coverage']['analyzed_files']==1 for row in rows)
    batch=core['batch']; rate_args=dict(selected=batch['selected_files'],enumerated=len(rows),read_ok=batch['read_ok_files'],applicable=applicable,completed=completed)
    data={
      'file-outcomes.json':core,
      'detector-coverage.json':{'byFormat':detector_counts(cells),'cells':cells,'caseCount':len(case_ids),'unit':'one detector invocation, not one file'},
      'coverage-denominators.json':{'controlled':rates(**rate_args,known_total=len(rows)),
        'unknownRepositoryPopulation':rates(**rate_args,known_total=None),
        'unknownApplicabilityAmongReadFiles':sum(row['expectedApplicable'] is None and row['coverage']['read_ok_files']==1 for row in rows),
        'definitions':{'enumeration':'observed selected + enumerated excluded / independently known file population; null when population unknown',
                       'applicability':'known applicable files from controlled input truth / read files; unknown applicability remains separate',
                       'completion':'analyzed / known applicable files, including applicable failures in denominator'},
        'controlledSelection':{'known':len(known),'enumerated':len(selected|excluded),'selected':len(selected),'excluded':len(excluded),'notEnumeratedKnown':len(not_enumerated)}},
      'exclusions.json':selection,
      'failure-retention.json':{'mixedDetectorFailure':core['mixedFailure'],'readFailure':[r for r in rows if r['expected_terminal']=='read_failed_files'],'malformedSibling':limits['malformedSibling'],'unknownNotSafe':True},
      'format-coverage.json':formats,
      'limit-outcomes.json':dict(limits,explicitWorkLimits=json.loads((source/'work-limits055.json').read_text())),
    }
    output.mkdir(parents=True,exist_ok=True)
    for name,value in data.items():
        path=output/name
        with path.open('x') as f:json.dump(value,f,ensure_ascii=False,indent=2);f.write('\n')
    (output/'enumeration-review.md').write_text(
      f'# TASK-055 列挙と分母\n\n管理した{len(known)}ファイルは選択{len(selected)}・列挙済み除外{len(excluded)}・ignoreによる未列挙{len(not_enumerated)}に分かれました。設定した除外方針は実際の除外件数ではありません。\n\n'
      '任意のリポジトリでは未知の総数を0に置き換えず、列挙率はnullとします。適用率は制御入力で適用を確認できた数、完了率はその集合での解析完了数から計算し、適用不明は別記します。失敗・未対応・未報告・打切りを成功にしません。\n\n'
      'ルートへのアクセスや列挙の致命的な失敗は空の成功結果を返さず、既存の安全なエラーを維持します。ignore解釈の失敗は不完全性として診断を保持します。\n')
    return data


if __name__=='__main__':
    import sys
    collect(sys.argv[1],sys.argv[2])
