#!/usr/bin/env python3
"""Update file handlers in saveit.html"""

with open('saveit.html', 'r', encoding='utf-8') as f:
    content = f.read()

# Update 1: Editor paste handler for file support
old_paste = """/* Paste inside editor — sanitize preserving color */
editorBody.addEventListener('paste',e=>{
  e.preventDefault();
  const cd=e.clipboardData;
  const html=cd.getData('text/html');
  const plain=cd.getData('text/plain');

  // Check for image blob
  for(const item of cd.items||[]){
    if(item.type.startsWith('image/')){
      const f=item.getAsFile();
      if(f){
        const reader=new FileReader();
        reader.onload=ev=>{
          document.execCommand('insertHTML',false,`<img src="${ev.target.result}" style="max-width:100%">`);
        };
        reader.readAsDataURL(f);
        return;
      }
    }
  }

  if(html){
    const clean=sanitizeHTML(html,{collapseBase64:true});
    document.execCommand('insertHTML',false,clean);
  } else if(plain){
    document.execCommand('insertText',false,plain);
  }
});"""

new_paste = """/* Paste inside editor — sanitize preserving color, handle files */
editorBody.addEventListener('paste',async e=>{
  const cd=e.clipboardData;
  const html=cd.getData('text/html');
  const plain=cd.getData('text/plain');

  // Check for files (including images)
  for(const item of cd.items||[]){
    if(item.type.startsWith('image/')){
      e.preventDefault();
      const f=item.getAsFile();
      if(f){
        const reader=new FileReader();
        reader.onload=ev=>{
          document.execCommand('insertHTML',false,`<img src="${ev.target.result}" style="max-width:100%">`);
        };
        reader.readAsDataURL(f);
        return;
      }
    } else if(item.kind==='file'){
      e.preventDefault();
      const f=item.getAsFile();
      if(f){
        const reader=new FileReader();
        reader.onload=async ev=>{
          try{
            const result=await invoke('store_file',{storagePath,filename:f.name,contents:ev.target.result.split(',')[1]});
            const path=result.path;
            document.execCommand('insertText',false,`[FILE](${path})`);
            toast(`File link inserted`,'success');
          }catch(err){toast(`Failed to store file: ${err}`,'error')}
        };
        reader.readAsDataURL(f);
        return;
      }
    }
  }

  if(html){
    e.preventDefault();
    const clean=sanitizeHTML(html,{collapseBase64:true});
    document.execCommand('insertHTML',false,clean);
  } else if(plain){
    document.execCommand('insertText',false,plain);
  }
});"""

if old_paste in content:
    content = content.replace(old_paste, new_paste)
    print("✅ Updated editor paste handler")
else:
    print("⚠️  Editor paste handler not found")

# Update 2: Editor drop handler for file support
old_drop = """editorBody.addEventListener('drop',e=>{
  e.preventDefault();
  const files=e.dataTransfer.files;
  const html=e.dataTransfer.getData('text/html');
  const plain=e.dataTransfer.getData('text/plain');

  // Check for image in files
  for(const f of files){
    if(f.type.startsWith('image/')){
      const reader=new FileReader();
      reader.onload=ev=>{
        insertHTML(`<img src="${ev.target.result}" style="max-width:100%">`);
      };
      reader.readAsDataURL(f);
      return;
    }
  }

  if(html){
    insertHTML(sanitizeHTML(html,{collapseBase64:true}));
  } else if(plain){
    insertText(plain);
  }
});"""

new_drop = """editorBody.addEventListener('drop',e=>{
  e.preventDefault();
  const files=e.dataTransfer.files;
  const html=e.dataTransfer.getData('text/html');
  const plain=e.dataTransfer.getData('text/plain');

  // Check for image in files
  for(const f of files){
    if(f.type.startsWith('image/')){
      const reader=new FileReader();
      reader.onload=ev=>{
        insertHTML(`<img src="${ev.target.result}" style="max-width:100%">`);
      };
      reader.readAsDataURL(f);
      return;
    } else {
      // Handle non-image files
      const reader=new FileReader();
      reader.onload=async ev=>{
        try{
          const result=await invoke('store_file',{storagePath,filename:f.name,contents:ev.target.result.split(',')[1]});
          const path=result.path;
          insertText(`[FILE](${path})`);
          toast(`File link inserted`,'success');
        }catch(err){toast(`Failed to store file: ${err}`,'error')}
      };
      reader.readAsDataURL(f);
      return;
    }
  }

  if(html){
    insertHTML(sanitizeHTML(html,{collapseBase64:true}));
  } else if(plain){
    insertText(plain);
  }
});"""

if old_drop in content:
    content = content.replace(old_drop, new_drop)
    print("✅ Updated editor drop handler")
else:
    print("⚠️  Editor drop handler not found")

# Save the updated content
with open('saveit.html', 'w', encoding='utf-8') as f:
    f.write(content)

print("✅ saveit.html updated successfully")
