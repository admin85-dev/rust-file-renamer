<script>
  import { invoke } from "@tauri-apps/api/core";
  import { open } from "@tauri-apps/plugin-dialog";

  let selectedDir = $state("");
  let files = $state([]);
  let editingFile = $state(null);
  let newName = $state("");
  let error = $state("");
  let success = $state("");

  async function pickFolder() {
    const selected = await open({
      directory: true,
      multiple: false,
    });

    if (selected) {
      selectedDir = selected;
      await loadFiles();
    }
  }

  async function loadFiles() {
    error = "";
    try {
      files = await invoke("read_directory", { path: selectedDir });
    } catch (e) {
      error = e;
    }
  }

  function startEditing(file) {
    editingFile = file;
    newName = file;
  }

  function cancelEditing() {
    editingFile = null;
    newName = "";
  }

  async function confirmRename() {
    error = "";
    success = "";
    try {
      await invoke("rename_file", {
        dir: selectedDir,
        oldName: editingFile,
        newName: newName,
      });
      success = `Renamed "${editingFile}" to "${newName}"`;
      editingFile = null;
      newName = "";
      await loadFiles();
    } catch (e) {
      error = e;
    }
  }
</script>

<main>
  <h1>File Renamer</h1>

  <button onclick={pickFolder}>Pick a Folder</button>

  {#if selectedDir}
    <p class="dir-path">📁 {selectedDir}</p>
  {/if}

  {#if error}
    <p class="error">{error}</p>
  {/if}

  {#if success}
    <p class="success">{success}</p>
  {/if}

  {#if files.length > 0}
    <ul>
      {#each files as file}
        <li>
          {#if editingFile === file}
            <input bind:value={newName} />
            <button onclick={confirmRename}>✓</button>
            <button onclick={cancelEditing}>✕</button>
          {:else}
            <span>{file}</span>
            <button onclick={() => startEditing(file)}>Rename</button>
          {/if}
        </li>
      {/each}
    </ul>
  {:else if selectedDir}
    <p>No files found in this folder.</p>
  {/if}
</main>

<style>
  main {
    padding: 2rem;
    font-family: Inter, Helvetica, Arial, sans-serif;
    max-width: 600px;
    margin: 0 auto;
  }

  h1 {
    margin-bottom: 1rem;
  }

  .dir-path {
    font-size: 0.85rem;
    color: #666;
    margin: 0.5rem 0 1rem;
    word-break: break-all;
  }

  ul {
    list-style: none;
    padding: 0;
    margin-top: 1rem;
  }

  li {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    padding: 0.5rem 0;
    border-bottom: 1px solid #eee;
  }

  span {
    flex: 1;
  }

  input {
    flex: 1;
    padding: 0.3rem 0.5rem;
    border: 1px solid #396cd8;
    border-radius: 4px;
    font-size: 0.95rem;
  }

  button {
    padding: 0.3rem 0.8rem;
    border: none;
    border-radius: 4px;
    background-color: #396cd8;
    color: white;
    cursor: pointer;
    font-size: 0.9rem;
  }

  button:hover {
    background-color: #2952a3;
  }

  .error {
    color: red;
    font-size: 0.9rem;
  }

  .success {
    color: green;
    font-size: 0.9rem;
  }
</style>
