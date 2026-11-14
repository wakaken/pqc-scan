"""Pinned Miri host/target separation and fail-closed libtest evidence parsing."""
import json
import re

NIGHTLY = 'nightly-2026-02-12'
# Cargo 0c9e687 also uses cfg(all()).runner for host build scripts. Keep
# those native while target test JSON continues through cargo-miri runner.
HOST_FLAGS = ['-Zhost-config', '-Ztarget-applies-to-host', '--config', 'host.runner=["/usr/bin/env"]']


def miri_command(verb, *args):
    return ['cargo', '+'+NIGHTLY, 'miri', verb, *HOST_FLAGS, *args]


def parse_test_events(log, known):
    known = list(known)
    if len(set(known)) != len(known):
        raise ValueError('duplicate registered test')
    states = {}
    for line in log.splitlines():
        if not line.startswith('{'):
            continue
        try:
            item = json.loads(line)
        except json.JSONDecodeError:
            continue  # truncated final event is not a completed test
        if item.get('type') != 'test':
            continue
        name, event = item.get('name'), item.get('event')
        if name not in known:
            raise ValueError('unregistered test event')
        if event == 'started':
            if name in states:
                raise ValueError('duplicate test start')
            states[name] = 'active'
        elif event == 'timeout':
            if states.get(name) != 'active':
                raise ValueError('timeout warning without active test')
        elif event in {'ok', 'failed', 'ignored'}:
            # libtest can emit ignored directly, without a start event.
            if states.get(name) != 'active' and not (event == 'ignored' and name not in states):
                raise ValueError('missing start or duplicate test completion')
            states[name] = {'ok':'passed', 'failed':'failed', 'ignored':'ignored'}[event]
        else:
            raise ValueError('unknown test event')
    return {key:[name for name in known if states.get(name)==key] for key in
            ['passed','failed','ignored','active']} | {'unseen':[name for name in known if name not in states]}


def classify_failure(status, log):
    if re.search(r'^error: Undefined Behavior:', log, re.MULTILINE):
        return 'Failed: UB'
    if status == 'timeout':
        return 'Partial: timeout'
    if re.search(r'^error: unsupported operation:', log, re.MULTILINE):
        return 'Unsupported'
    if 'assertion failed' in log or '... FAILED' in log:
        return 'Failed: assertion'
    return 'Unknown: execution error'
