import './style.css';
import { invoke } from '@tauri-apps/api/core';

const $ = (id) => document.getElementById(id);
const dialog = $('editor');
let apps = [];

async function refresh() {
  try {
    apps = await invoke('list_apps');
    render();
  } catch (e) {
    $('formError').textContent = String(e);
  }
}
function render() {
  $('appCount').textContent = `${apps.length} 个应用`;
  $('empty').style.display = apps.length ? 'none' : 'block';
  $('apps').innerHTML = apps.map(app => `
    <article class="app-card">
      <div class="app-card-top">
        <div class="app-icon">${escapeHtml(app.icon_text || 'WEB')}</div>
        <div style="min-width:0"><h4>${escapeHtml(app.name)}</h4><div class="url">${escapeHtml(app.url)}</div></div>
      </div>
      <div class="card-actions">
        <button class="launch" data-action="launch" data-id="${app.id}">启动应用 ↗</button>
        <button data-action="edit" data-id="${app.id}">编辑</button>
        <button data-action="delete" data-id="${app.id}">删除</button>
      </div>
    </article>`).join('');
}
function escapeHtml(value) {
  return String(value).replace(/[&<>"']/g, c => ({'&':'&amp;','<':'&lt;','>':'&gt;','"':'&quot;',"'":'&#39;'}[c]));
}
function openEditor(app = null) {
  $('appForm').reset();
  $('formError').textContent = '';
  $('dialogTitle').textContent = app ? '编辑网站' : '添加网站';
  $('appId').value = app?.id || '';
  $('name').value = app?.name || '';
  $('url').value = app?.url || '';
  $('width').value = app?.width || 1440;
  $('height').value = app?.height || 900;
  $('iconText').value = app?.icon_text || 'WEB';
  $('resizable').checked = app?.resizable ?? true;
  $('fullscreen').checked = app?.fullscreen ?? false;
  dialog.showModal();
}
$('addBtn').addEventListener('click', () => openEditor());
$('closeDialog').addEventListener('click', () => dialog.close());
$('cancelBtn').addEventListener('click', () => dialog.close());
$('refreshBtn').addEventListener('click', refresh);

$('appForm').addEventListener('submit', async (event) => {
  event.preventDefault();
  const payload = {
    id: $('appId').value || crypto.randomUUID(),
    name: $('name').value.trim(),
    url: $('url').value.trim(),
    width: Number($('width').value),
    height: Number($('height').value),
    icon_text: $('iconText').value.trim() || 'WEB',
    resizable: $('resizable').checked,
    fullscreen: $('fullscreen').checked
  };
  try {
    await invoke('save_app', { app: payload });
    dialog.close();
    await refresh();
  } catch (e) {
    $('formError').textContent = String(e);
  }
});

$('apps').addEventListener('click', async (event) => {
  const button = event.target.closest('button[data-action]');
  if (!button) return;
  const app = apps.find(item => item.id === button.dataset.id);
  if (!app) return;
  try {
    if (button.dataset.action === 'launch') await invoke('launch_app', { id: app.id });
    if (button.dataset.action === 'edit') openEditor(app);
    if (button.dataset.action === 'delete' && confirm(`确定删除“${app.name}”吗？`)) {
      await invoke('delete_app', { id: app.id });
      await refresh();
    }
  } catch (e) {
    alert(String(e));
  }
});

refresh();
