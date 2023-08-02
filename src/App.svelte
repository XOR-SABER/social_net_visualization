<script>
  import Graph from './lib/Graph.svelte';
  import { appWindow } from '@tauri-apps/api/window';
  import { invoke } from '@tauri-apps/api/tauri';
  import { save, open} from '@tauri-apps/api/dialog';

  //This handles opening files..
  appWindow.listen("openfile", () => {
    call_open();
  });

  //This handles saving files.. 
  appWindow.listen("savefile", () => {
    call_save();
  });

  const call_open = async() => {
    try {
      let open_path = open({
        multiple: false,
        filters: [{
          name: 'Image',
          extensions: ['png', 'jpeg']
        }]
      });
      await invoke("open_graph", { open_path });
    } catch (err) {
      console.log(err);
    }
  }

  const call_save = async() => {
    try {
      let save_path = save();
      await invoke("save_graph", {save_path});
    } catch (err) {
      console.log(err);
    }
  }

</script>

<main class="container">
  <Graph/>
</main>

<!-- <style>
  
  .logo.vite:hover {
    filter: drop-shadow(0 0 2em #747bff);
  }

  .logo.svelte:hover {
    filter: drop-shadow(0 0 2em #ff3e00);
  }
</style> -->
