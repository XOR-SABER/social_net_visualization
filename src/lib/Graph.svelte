<!-- Graph.svelte -->
<script>
  import { invoke } from "@tauri-apps/api/tauri";
  import GraphUtills from "./GraphUtills.svelte";

  // This the modifiable variables
  let width = 720;
  let height = 720;

  // This the data we initlaize the graph with
  export let Data;

  //Cache all the connections
  const connection_cache = new Map();

  async function add_into_hash(str) {
    const connections = await invoke("send_graph_connections", { id: str });
    if (connections.length > 0) connection_cache.set(str, connections);
  }

  const split_data = (Data) => {
    let temp = {
      NodeElms: [],
      LinkElms: [],
    };

    // Split them!
    Data.forEach((pair) => {
      const [first, second] = pair;
      temp.NodeElms.push(first);
      add_into_hash(first);
      temp.LinkElms.push(second);
    });

    return temp;
  };

  const parsed_data = split_data(Data);

  const Processed_data = (temp) => {
    // Create retval
    const graphnodes = temp.NodeElms.map((NodeElements) => ({
      id: NodeElements,
      NodeElements,
    }));

    const linknodes = [];
    for (let i = 0; i < temp.NodeElms.length; i++) {
      temp.LinkElms[i].forEach((str) => {
        linknodes.push({ source: temp.NodeElms[i], target: str }); // Fix the format of the linknodes.push() call
      });
    }

    let retval = {
      nodes: graphnodes,
      links: linknodes,
    };
    return retval;
  };

  const graphData = Processed_data(parsed_data);
</script>

<div>
  <GraphUtills {width} {height} {graphData} {connection_cache} {parsed_data} />
</div>
