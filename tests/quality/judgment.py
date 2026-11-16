"""Compare frozen judgment expectations with actual public-interface observations."""
import json
import math
import re


def select(value, pointer):
    if not isinstance(pointer,str) or (pointer and not pointer.startswith('/')):
        raise ValueError('Invalid oracle pointer')
    for part in pointer.split('/')[1:] if pointer else []:
        if re.search(r'~(?![01])',part):raise ValueError('Invalid oracle escape')
        key=part.replace('~1','/').replace('~0','~')
        if isinstance(value,list):
            if not re.fullmatch(r'0|[1-9][0-9]*',key):raise KeyError(key)
            value=value[int(key)]
        else:value=value[key]
    return value


def equal(a,b):
    # JSON booleans and numbers are different facts (True must not equal 1).
    return json.dumps(a,sort_keys=True,allow_nan=False)==json.dumps(b,sort_keys=True,allow_nan=False)


def canonical_cases(cases):
    """Replace historical characterization only when an identical input is reassessed."""
    indexed={c['id']:c for c in cases}
    if len(indexed)!=len(cases):raise ValueError('Duplicate judgment case')
    replaced=set()
    for case in cases:
        previous=case.get('reassesses')
        if previous is None:continue
        original=indexed.get(previous)
        if (original is None or previous in replaced or original.get('reassesses') is not None
                or original['checks'] or not case['checks']
                or not equal(original['input'],case['input'])):
            raise ValueError('Invalid or changed-input reassessment')
        replaced.add(previous)
    return [c for c in cases if c['id'] not in replaced]


def compare(oracle,observations):
    cases=oracle['cases'];ids=[c['id'] for c in cases];seen=[o['id'] for o in observations]
    if len(set(ids))!=len(ids) or len(set(seen))!=len(seen) or set(ids)!=set(seen):
        raise ValueError('Missing, duplicate or extra judgment observations')
    indexed={o['id']:o['actual'] for o in observations};correct=incorrect=unassessed=0;mismatches=[];rows=[]
    for case in cases:
        checks=case['checks'];failed=[]
        for check in checks:
            operation=check['operation'];expected=check['expected']
            if operation not in {'equals','contains','not_contains','approximately','absent'}:
                raise ValueError('Unsupported oracle operation')
            try:actual=select(indexed[case['id']],check['pointer'])
            except (KeyError,IndexError,TypeError):
                if operation=='absent':continue
                failed.append(dict(pointer=check['pointer'],reason='Missing actual property',expected=expected));continue
            if operation=='equals':passed=equal(actual,expected)
            elif operation=='contains':passed=isinstance(actual,list) and any(equal(x,expected) for x in actual)
            elif operation=='not_contains':passed=isinstance(actual,list) and not any(equal(x,expected) for x in actual)
            elif operation=='absent':passed=False
            elif operation=='approximately':
                tolerance=check['tolerance']
                if type(tolerance) not in (int,float) or not math.isfinite(tolerance) or not 0<tolerance<=1e-6:
                    raise ValueError('Invalid numeric transport tolerance')
                passed=(type(actual) in (int,float) and type(expected) in (int,float)
                        and math.isfinite(actual) and math.isfinite(expected) and abs(actual-expected)<=tolerance)
            else:raise ValueError('Unsupported oracle operation')
            if not passed:failed.append(dict(pointer=check['pointer'],actual=actual,expected=expected,reason='Judgment differs'))
        status='Not Tested' if not checks else 'Failed' if failed else 'Verified'
        if not checks:unassessed+=1
        elif failed:incorrect+=1;mismatches.append(dict(id=case['id'],differences=failed))
        else:correct+=1
        rows.append(dict(id=case['id'],kind=case.get('kind','synthetic-control'),requirements=case.get('requirements',[]),
            basis=case.get('basis','synthetic-control'),status=status,assertions=len(checks)))
    evaluated=correct+incorrect
    return dict(correct=correct,incorrect=incorrect,unassessed=unassessed,evaluated=evaluated,
        correctnessRate=correct/evaluated if evaluated else None,rows=rows,mismatches=mismatches,
        interpretation='Agreement with this frozen judgment table only; not matcher accuracy, runtime execution, trust validation or universal cryptographic safety')
