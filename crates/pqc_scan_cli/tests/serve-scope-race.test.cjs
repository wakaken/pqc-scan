const {test}=require('node:test');
const assert=require('node:assert/strict');
const {readFileSync}=require('node:fs');
const {join}=require('node:path');
const vm=require('node:vm');
// Run the shipped continuation/state transitions with a delayed HTTP boundary.
// No grouping, request sequencing or auto-navigation logic is reimplemented here.
const source=readFileSync(join(__dirname,'../src/serve-migration.js'),'utf8');
function implementation(name) {
 const start=source.indexOf(`    ${name==='refreshScope'?'async ':''}function ${name}(`);
 assert.ok(start>=0);
 const end=source.indexOf('\n    }',start)+6;
 return source.slice(start,end);
}
async function authoring() {
 let resolve;
 const delayed=new Promise(r=>{resolve=r});
 const c={mode:'new',loading:false,scopePending:false,scopeSequence:0,version:1,editorRevision:0,navigationSequence:0,
  progressPanel:{open:false,hidden:true,close(){this.open=false}},progressShown:0,generationProgress(){c.progressShown++;c.progressPanel.open=true;},
  groupId:null,individualId:null,selected:new Set(['1']),scopeKeys:new Set(['scope']),scopeData:null,
  targetById:new Map(),drafts:new Map(),input:{sha256:'digest'},job:{id:1},moduleSelect:{value:'core'},summary:{cohorts:[]},
  store:{saveNew(){},saveCommon(){},saveIndividual(){},effective(){return []}},
  read:()=>({}),draftKey:()=> 'new',validateDraft:()=>true,
  resetConfirmation(){c.editorRevision++},render(){},renderCohorts(){},updateQuestions(){},
  switchEditor(next){c.cancelScope();c.mode=next}, api:()=>delayed, reportError(e){throw e}, generated:0,
  reviewRequest(){c.generated++}, t:ja=>ja,
 };
 vm.createContext(c);
 vm.runInContext(['cancelScope','refreshScope','saveDraft'].map(implementation).join('\n'),c);
 assert.equal(c.saveDraft(),true);
 return {c, async finish(){resolve({input:{sha256:'digest'},cohorts:[],targets:[]});await new Promise(r=>setImmediate(r));}};
}
test('Next generates only from its applied scope reply',async()=>{
 const {c,finish}=await authoring();assert.equal(c.generated,0);assert.equal(c.scopePending,true);assert.equal(c.progressShown,1);assert.equal(c.progressPanel.open,true);
 await finish();assert.equal(c.generated,1);assert.equal(c.scopePending,false);
});
test('a discarded scope reply cannot generate after returning to an editor',async()=>{
 const {c,finish}=await authoring();c.switchEditor('group');await finish();assert.equal(c.generated,0);assert.equal(c.mode,'group');
});
test('new input or navigation during scope loading cancels the saved Next continuation',async()=>{
 for(const field of ['editorRevision','navigationSequence']){
  const {c,finish}=await authoring();c[field]++;await finish();assert.equal(c.generated,0,field);
 }
});
test('a newer request supersedes both selection and auto-navigation from the old reply',async()=>{
 const {c,finish}=await authoring();c.cancelScope();c.selected.add('manual');await finish();
 assert.deepEqual([...c.selected],['manual']);assert.equal(c.generated,0);
});

test('delete and undo wait until pending membership has settled',()=>{
 const nodes=new Map();
 const node=(tag,text='')=>({tag,text,children:[],disabled:false,append(...v){this.children.push(...v)},replaceChildren(){this.children=[]}});
 const group={id:'g1',name:'saved',ids:['1'],exceptions:new Map()};
 const c={groups:[group],removedUses:[{group}],allTotal:3,loading:false,scopePending:true,
  document:{createElement:tag=>node(tag)}, textNode:node,t:ja=>ja,button:(text,callback)=>({...node('button',text),callback}),
  $:id=>{if(!nodes.has(id)) nodes.set(id,node('section'));return nodes.get(id)},store:{count:()=>1},
 };
 vm.createContext(c);vm.runInContext(implementation('renderGroups'),c);
 c.renderGroups();
 const controls=()=>nodes.get('groups-list').children.flatMap(row=>row.children).filter(n=>['削除','元に戻す'].includes(n.text));
 assert.equal(controls().length,2);assert.ok(controls().every(n=>n.disabled));
 c.scopePending=false;c.renderGroups();assert.ok(controls().every(n=>!n.disabled));
});

test('Response tab opens a landing screen without generation; explicit imported preparation keeps context and shows validation',()=>{
 const node=()=>({hidden:false,children:[],dataset:{},setAttribute(){},focus(){},append(...n){this.children.push(...n)},replaceChildren(...n){this.children=n}});
 const nodes=new Map();const get=id=>{if(!nodes.has(id))nodes.set(id,{value:id==='context-mode'?'import_context':''});return nodes.get(id)};
 const c={loading:false,navigationSequence:0,stageView:'input',overviewPanel:node(),contextStage:node(),reportPanel:node(),review:node(),heading:node(),stageNav:node(),mountedReport:null,
 t:ja=>ja,textNode:()=>node(),button:(name,action)=>({...node(),name,action}),selectedIds:()=>[],store:{count:()=>0},$:get,saved:0,requests:[],saveDraft(){c.saved++},reviewRequest(without){c.requests.push({without,inputVisible:!c.contextStage.hidden})}};
 c.heading.querySelector=()=>node();vm.createContext(c);vm.runInContext(implementation('stage'),c);
 c.stage('input');c.stageNav.children.find(n=>n.name==='対応項目').action();
 assert.equal(c.reportPanel.children[0].dataset.en,'Migration review items');assert.equal(c.reportPanel.children[2].dataset.en,'Prepare review items');
 assert.equal(c.stageView,'report');assert.equal(c.requests.length,0);assert.equal(c.saved,0);
 c.reportPanel.children.find(n=>n.name==='確認事項を作成').action();
 assert.equal(c.requests[0].without,false);assert.equal(c.requests[0].inputVisible,true);
});
test('explicit schedule changes select duration or goal without rewriting imported values during load',()=>{
 const values={'lead-time':{value:'specified'},'lead-time-value':{value:'123'},'goal-month':{value:'specified'},'goal-month-value':{value:'2030-09'}};
 const c={$:id=>values[id]};vm.createContext(c);vm.runInContext(implementation('chooseSchedule'),c);
 assert.equal(values['lead-time-value'].value,'123');assert.equal(values['goal-month-value'].value,'2030-09');
 c.chooseSchedule('goal-month');assert.equal(values['lead-time'].value,'');assert.equal(values['lead-time-value'].value,'');assert.equal(values['goal-month-value'].value,'2030-09');
 values['lead-time'].value='specified';c.chooseSchedule('lead-time');assert.equal(values['goal-month'].value,'');assert.equal(values['goal-month-value'].value,'');
});
