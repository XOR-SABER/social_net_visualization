<!-- Graph.svelte -->
<script>
  import { onMount } from "svelte";
  import * as d3 from "d3";

  export let Data;

  console.log(Data);

  const NodeElements = [];
  const LinkElements = [];

  Data.forEach((pair) => {
    const [first, second] = pair;
    NodeElements.push(first);
    LinkElements.push(second);
  });

  console.log(NodeElements);
  console.log(LinkElements);

  const graphnodes = NodeElements.map((NodeElements) => ({
    id: NodeElements,
    NodeElements,
  }));
  const linknodes = [];

  for (let i = 0; i < NodeElements.length; i++) {
    LinkElements[i].forEach((str) => {
      linknodes.push({ source: NodeElements[i], target: str }); // Fix the format of the linknodes.push() call
    });
  }

  console.log(linknodes);

  const graphData = {
    nodes: graphnodes,
    links: linknodes, // Use the generated linknodes array
  };

  console.log(graphData);

  // const graphData = {
  //   nodes : graphnodes,
  //   links: [
  //     { source: 'node1', target: 'node2' },
  //     { source: 'node1', target: 'node3' },
  //     { source: 'node2', target: 'node3' },
  //     { source: 'node3', target: 'node4' },
  //   ],
  // };

  // Convert the source and target strings to actual node objects

  let width = 720;
  let height = 720;

  let svg;
  let simulation;

  onMount(() => {
    const linkData = graphData.links.map((link) => ({
      ...link,
      source: graphData.nodes.find((node) => node.id === link.source),
      target: graphData.nodes.find((node) => node.id === link.target),
    }));
    // D3.js code here (same as in your previous code)

    // Your D3.js code here
    svg = d3
      .select("#graph")
      .append("svg")
      .attr("width", width)
      .attr("height", height);

    simulation = d3
      .forceSimulation()
      .force(
        "link",
        d3.forceLink().id((d) => d.id)
      )
      .force("charge", d3.forceManyBody().strength(-1500))
      .force("center", d3.forceCenter(width / 2, height / 2));

    const link = svg
      .append("g")
      .selectAll("line")
      .data(linkData) // Use the updated linkData array
      .enter()
      .append("line")
      .style("stroke", "#464F51")
      .style("stroke-width", 2);

    const tooltip = d3
      .select("body")
      .append("div")
      .attr("class", "tooltip")
      .style("opacity", 0)
      .style("position", "absolute")
      .style("pointer-events", "none")
      .style("background-color", "rgba(0, 0, 0, 0.7)")
      .style("color", "white")
      .style("padding", "5px")
      .style("border-radius", "3px")
      .style("font-size", "12px");

    const node = svg
      .append("g")
      .selectAll("circle")
      .data(graphData.nodes)
      .enter()
      .append("circle")
      .attr("r", 10)
      .style("fill", "#724cf9")
      .call(
        d3
          .drag()
          .on("start", dragStarted)
          .on("drag", dragging)
          .on("end", dragEnded)
      )
      .on("mousemove", handleMouseMove) // New event listener
      .on("mouseout", handleMouseOut); // New event listener

    function handleMouseMove(event, d) {
      const [mouseX, mouseY] = d3.pointer(event);
      tooltip
        .style("opacity", 0.9)
        .html(d.id) // Use the node's name from data
        .style("left", mouseX + 10 + "px")
        .style("top", mouseY - 20 + "px");
    }

    function handleMouseOut() {
      tooltip.style("opacity", 0);
    }

    simulation.nodes(graphData.nodes).on("tick", ticked);
    simulation.force("link").links(linkData); // Use the updated linkData array

    function ticked() {
      link
        .attr("x1", (d) => d.source.x)
        .attr("y1", (d) => d.source.y)
        .attr("x2", (d) => d.target.x)
        .attr("y2", (d) => d.target.y);

      node.attr("cx", (d) => d.x).attr("cy", (d) => d.y);
    }

    function dragStarted(event, d) {
      if (!event.active) simulation.alphaTarget(0.1).restart();
      d.fx = d.x;
      d.fy = d.y;
    }

    function dragging(event, d) {
      handleMouseOut();
      d.fx = event.x;
      d.fy = event.y;
    }

    function dragEnded(event, d) {
      if (!event.active) simulation.alphaTarget(0.001);
      d.fx = null;
      d.fy = null;
    }

    function handleResize() {
      width = window.innerWidth;
      height = window.innerHeight;

      // Update SVG width and height
      svg.attr("width", width).attr("height", height);

      // Update force center
      simulation.force("center", d3.forceCenter(width / 2, height / 2));

      // Restart the simulation to update the positions of nodes and links
      simulation.alpha(1).restart();
    }

    // Add window resize event listener
    window.addEventListener("resize", handleResize);
  });
</script>

<div id="graph">
  <!-- The graph will be rendered inside this div -->
</div>
