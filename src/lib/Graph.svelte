<!-- Graph.svelte -->
<script>
  import { invoke } from "@tauri-apps/api/tauri";
  import GraphUtills from "./GraphUtills.svelte";

  // This the modifiable variables
  let width = window.innerWidth;
  let height = window.innerHeight;

  // This the data we initlaize the graph with
  export let Data;

  const split_data = (Data) => {
    let temp = {
      NodeElms: [],
      LinkElms: [],
    };

    // Split them!
    Data.forEach((pair) => {
      const [first, second] = pair;
      temp.NodeElms.push(first);
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
  <GraphUtills {width} {height} {graphData} {parsed_data} />
</div>
