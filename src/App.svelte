<script>
  import Graph from "./lib/Graph.svelte";
  import { appWindow } from "@tauri-apps/api/window";
  import { invoke } from "@tauri-apps/api/tauri";
  import { save, open } from "@tauri-apps/api/dialog";
  import { onMount } from "svelte";

  let graphData;

  onMount(() => {
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

    const call_open = async () => {
      try {
        const open_path = await open({
          multiple: false,
          filters: [
            {
              name: "File",
              extensions: ["txt", "dat"],
            },
          ],
        });
        graphData = null;
        const check = await invoke("open_graph", { path: open_path });
        graphData = await invoke("send_graph_nodes");
        if (!check) throw "Backend issue";
      } catch (err) {
        console.log(err);
      }
    };

    const call_as_save = async () => {
      try {
        const save_as_path = await save();
        const check = await invoke("save_graph", { save_as_path });
        if (!check) throw "Backend issue";
      } catch (err) {
        console.log(err);
      }
    };

    const call_save = async () => {
      try {
        const save_path = await open({
          directory: true,
        });
        const check = await invoke("save_graph", { save_path });
        if (!check) throw "Backend issue";
      } catch (err) {
        console.log(err);
      }
    };
  });
</script>

<main class="container">
  {#if !graphData}
    <p>Waiting for graph data...</p>
  {:else}
    <Graph Data={graphData} />
  {/if}
</main>
