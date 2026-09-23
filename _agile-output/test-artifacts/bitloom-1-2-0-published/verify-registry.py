from pathlib import Path
import json,urllib.request,concurrent.futures,datetime,hashlib,tarfile,io,tomllib,copy
out=Path('/tmp/bitloom-1-2-0-publish');journal=json.loads((out/'journal.json').read_text());assert journal.get('complete')
def members(payload):
 with tarfile.open(fileobj=io.BytesIO(payload)) as t:return {m.name:t.extractfile(m).read() for m in t if m.isfile()}
def get(url):
 with urllib.request.urlopen(urllib.request.Request(url,headers={'User-Agent':'Bitloom-authorized-release-verification'}),timeout=30) as r:return r.read()
def fetch(item):
 name=item['name'];url='https://crates.io/api/v1/crates/'+name+'/1.2.0';data=json.loads(get(url))['version'];payload=get(url+'/download')
 assert data['num']=='1.2.0' and not data['yanked'],name
 assert hashlib.sha256(payload).hexdigest()==data['checksum'],name
 return name,(data,payload,url)
with concurrent.futures.ThreadPoolExecutor(max_workers=5) as pool:downloaded=dict(pool.map(fetch,journal['packages']))
results=[]
for item in journal['packages']:
 name=item['name'];data,payload,url=downloaded[name];local=Path('target/package')/f'{name}-1.2.0.crate';a,b=members(payload),members(local.read_bytes());assert a.keys()==b.keys(),name
 differences=[]
 for key in a:
  if key.endswith('/.cargo_vcs_info.json'):
   av,bv=json.loads(a[key]),json.loads(b[key]);assert av['git']['sha1']==journal['source_commit'] and not av['git'].get('dirty',False),(name,av)
   assert av['path_in_vcs']==bv['path_in_vcs'],(name,key)
   if av!=bv:differences.append({'file':key,'registry':av,'local':bv,'kind':'candidate versus clean release VCS metadata'})
  elif key.endswith('/Cargo.lock'):
   av,bv=tomllib.loads(a[key].decode()),tomllib.loads(b[key].decode());normalized_a,normalized_b=copy.deepcopy(av),copy.deepcopy(bv)
   checks=[]
   for doc,is_public in [(normalized_a,True),(normalized_b,False)]:
    for dep in doc['package']:
     if dep['name'] in downloaded:
      assert dep['version']=='1.2.0',(name,dep)
      if 'checksum' in dep:
       if is_public:
        assert dep['checksum']==downloaded[dep['name']][0]['checksum'],(name,dep)
        checks.append({'name':dep['name'],'version':dep['version'],'registry_checksum':dep['checksum']})
       dep.pop('checksum')
   assert normalized_a==normalized_b,(name,key,'dependency versions or sources differ')
   if av!=bv:differences.append({'file':key,'kind':'family checksum update from candidate archives to published archives','verified_registry_dependencies':checks})
  else:assert a[key]==b[key],(name,key)
 results.append({'name':name,'version':data['num'],'registry_checksum':data['checksum'],'local_current_checksum':hashlib.sha256(local.read_bytes()).hexdigest(),'local_after_publish_checksum':item['archive_sha256'],'yanked':data['yanked'],'created_at':data['created_at'],'url':url,'source_and_manifest_members_identical':True,'dependency_versions_identical':True,'metadata_differences':differences})
(out/'registry-confirmation.json').write_text(json.dumps({'checked_at':datetime.datetime.now(datetime.timezone.utc).isoformat(),'passed':True,'source_commit':journal['source_commit'],'packages':results},indent=2)+'\n')
print('All ten registry checksums, clean release commits, source files and lock dependencies verified.')
