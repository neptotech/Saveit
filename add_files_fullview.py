#!/usr/bin/env python3
"""Add files type support to renderFullview"""

with open('saveit.html', 'r', encoding='utf-8') as f:
    content = f.read()

# Update renderFullview to add files type support
old_section = """  } else if(item.type==='link'){
    const box=document.createElement('div');box.className='fv-link-box';
    box.innerHTML=`
      ${item.title?`<div class="fv-link-title">${escHTML(item.title)}</div>`:''}
      <a class="fv-link-url" href="${escHTML(item.url)}" target="_blank" rel="noopener">${escHTML(item.url)}</a>
      <a href="${escHTML(item.url)}" target="_blank" rel="noopener" style="display:inline-block;margin-top:6px;padding:6px 14px;background:var(--accent);color:#fff;border-radius:6px;font-size:13px;text-decoration:none">Open ↗</a>`;
    fvContent.appendChild(box);
  } else {"""

new_section = """  } else if(item.type==='link'){
    const box=document.createElement('div');box.className='fv-link-box';
    box.innerHTML=`
      ${item.title?`<div class="fv-link-title">${escHTML(item.title)}</div>`:''}
      <a class="fv-link-url" href="${escHTML(item.url)}" target="_blank" rel="noopener">${escHTML(item.url)}</a>
      <a href="${escHTML(item.url)}" target="_blank" rel="noopener" style="display:inline-block;margin-top:6px;padding:6px 14px;background:var(--accent);color:#fff;border-radius:6px;font-size:13px;text-decoration:none">Open ↗</a>`;
    fvContent.appendChild(box);
  } else if(item.type==='files'){
    const box=document.createElement('div');box.className='fv-link-box';
    const filename=item.filename||item.path.split('\\\\').pop()||'File';
    box.innerHTML=`
      <div class="fv-link-title">📁 ${escHTML(filename)}</div>
      <div class="fv-link-url" style="font-size:12px;color:var(--text-dim);margin-bottom:8px">${escHTML(item.path||'')}</div>`;
    const openBtn=document.createElement('button');
    openBtn.textContent='Open File';
    openBtn.style.cssText='display:inline-block;padding:6px 14px;background:var(--accent);color:#fff;border:none;border-radius:6px;font-size:13px;cursor:pointer';
    openBtn.addEventListener('click',async()=>{
      try{
        await invoke('open_file',{storagePath,path:item.path});
      }catch(e){toast(`Failed to open file: ${e}`,'error')}
    });
    box.appendChild(openBtn);
    fvContent.appendChild(box);
  } else {"""

if old_section in content:
    content = content.replace(old_section, new_section)
    print("✅ Updated renderFullview for files type")
else:
    print("⚠️  renderFullview section not found")

with open('saveit.html', 'w', encoding='utf-8') as f:
    f.write(content)

print("✅ saveit.html updated successfully")
