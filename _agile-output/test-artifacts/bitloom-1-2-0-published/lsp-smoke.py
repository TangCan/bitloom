"""用真实LSP initialize/shutdown验证安装后二进制及版本。"""
import subprocess,sys,json,os,select,time
from pathlib import Path
s=Path(sys.argv[1]);p=subprocess.Popen([str(s/'install/bin/bitloom-lsp')],stdin=subprocess.PIPE,stdout=subprocess.PIPE,stderr=subprocess.PIPE)
def send(obj):
 body=json.dumps(obj).encode();p.stdin.write(f'Content-Length: {len(body)}\r\n\r\n'.encode()+body);p.stdin.flush()
def receive():
 data=b'';end=time.monotonic()+15
 while time.monotonic()<end:
  if select.select([p.stdout],[],[],max(0,end-time.monotonic()))[0]:
   chunk=os.read(p.stdout.fileno(),4096);assert chunk,'unexpected EOF';data+=chunk
   if b'\r\n\r\n' in data:
    header,body=data.split(b'\r\n\r\n',1);size=int(header.decode().split(':',1)[1].strip())
    if len(body)>=size:return json.loads(body[:size])
 raise TimeoutError('LSP response')
try:
 send({'jsonrpc':'2.0','id':1,'method':'initialize','params':{'processId':None,'rootUri':None,'capabilities':{}}})
 result=receive();assert result['id']==1 and result['result']['serverInfo']['version']=='1.2.0',result
 send({'jsonrpc':'2.0','id':2,'method':'shutdown','params':None});shutdown=receive();assert shutdown['id']==2 and shutdown.get('result') is None and 'error' not in shutdown,shutdown
 send({'jsonrpc':'2.0','method':'exit','params':None});assert p.wait(timeout=10)==0
 print(json.dumps({'passed':True,'initialize':result,'shutdown':shutdown,'exit':0},indent=2))
finally:
 if p.poll() is None:p.kill();p.wait()
