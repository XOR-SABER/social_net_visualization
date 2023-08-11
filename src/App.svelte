<script>
  import Graph from './lib/Graph.svelte';
  import { appWindow } from '@tauri-apps/api/window';
  import { invoke } from '@tauri-apps/api/tauri';
  import { save, open} from '@tauri-apps/api/dialog';

  let receivedGraphData; // Initialize with no data

  //This handles opening files..
  appWindow.listen("openfile", () => {
    call_open();
  });

  //This handles saving files.. 
  appWindow.listen("saveasfile", () => {
    call_as_save();
  });

  //This handles saving to directory.. 
  appWindow.listen("savefile", () => {
    call_save();
  });

  const call_open = async() => {
    try {
      const open_path = await open({
        multiple: false,
        filters: [{
          name: 'File',
          extensions: ['txt', 'dat']
        }]
      });
      const check = await invoke("open_graph", { path: open_path});
      receivedGraphData = await invoke("send_graph_nodes");
      console.log(receivedGraphData);
      if(!check) throw("Backend issue")
    } catch (err) {
      console.log(err);
    }
  }

  const call_as_save = async() => {
    try {
      const save_as_path = await save();
      const check = await invoke("save_graph", {save_as_path});
      if(!check) throw("Backend issue")
    } catch (err) {
      console.log(err);
    }
  }

  const call_save = async() => {
    try {
      const save_path = await open( {
        directory: true,
      });
      const check = await invoke("save_graph", {save_path});
      if(!check) throw("Backend issue")
    } catch (err) {
      console.log(err); 
    }
  }

</script>

<main class="container">
{#if !receivedGraphData}
<p>Waiting for graph data...</p>
{:else}
  
  <Graph Data={receivedGraphData} />
{/if}
</main>

<!-- <style>
  
  .logo.vite:hover {
    filter: drop-shadow(0 0 2em #747bff);
  }

  .logo.svelte:hover {
    filter: drop-shadow(0 0 2em #ff3e00);
  }
</style> -->
