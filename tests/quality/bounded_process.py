"""Finite POSIX child supervision for isolated studies, not a product launcher."""
import math
import os
from pathlib import Path
import resource
import signal
import subprocess
import time


def execute(command,output,*,timeout=2,cpu_seconds=2):
    if not math.isfinite(timeout) or timeout<=0 or type(cpu_seconds) is not int or cpu_seconds<=0:
        raise ValueError('finite positive wall and CPU limits required')
    output=Path(output);output.mkdir(parents=True,exist_ok=False)
    def limits():
        resource.setrlimit(resource.RLIMIT_CORE,(0,0))
        resource.setrlimit(resource.RLIMIT_FSIZE,(2*1024**2,2*1024**2))
        resource.setrlimit(resource.RLIMIT_CPU,(cpu_seconds,cpu_seconds+1))
    start=time.monotonic();timed_out=False;terminated=False
    with (output/'stdout.log').open('wb') as stdout,(output/'stderr.log').open('wb') as stderr:
        process=subprocess.Popen(command,stdout=stdout,stderr=stderr,start_new_session=True,preexec_fn=limits)
        try:
            try:code=process.wait(timeout=timeout)
            except subprocess.TimeoutExpired:
                timed_out=True;os.killpg(process.pid,signal.SIGKILL);terminated=True;code=process.wait()
        finally:
            # Even a successful leader may have left descendants in its group.
            if not terminated:
                try:os.killpg(process.pid,signal.SIGKILL);terminated=True
                except ProcessLookupError:pass
            process.wait()
    status='timeout' if timed_out else {0:'completed',42:'panic',86:'invariant'}.get(code,'signal' if code<0 else 'error')
    return {'command':command,'status':status,'returncode':code,'wallSeconds':time.monotonic()-start,
            'wallLimitSeconds':timeout,'cpuLimitSeconds':cpu_seconds,'fileLimitBytes':2*1024**2,'coreDumpBytes':0,
            'groupTerminationSent':terminated,'stdoutBytes':(output/'stdout.log').stat().st_size,'stderrBytes':(output/'stderr.log').stat().st_size}
