<!-- Graph.svelte -->
<script>
  import { onMount } from 'svelte';
  import * as d3 from 'd3';

  // const graphData = {
  //   nodes: [
  //     { id: 'node1', name: 'Node 1' },
  //     { id: 'node2', name: 'Node 2' },
  //     { id: 'node3', name: 'Node 3' },
  //     { id: 'node4', name: 'Node 4' },
  //   ],
  //   links: [
  //     { source: 'node1', target: 'node2' },
  //     { source: 'node1', target: 'node3' },
  //     { source: 'node2', target: 'node3' },
  //     { source: 'node3', target: 'node4' },
  //   ],
  // };

  // Convert the source and target strings to actual node objects
  const graphData = null;

  let width = 720;
  let height = 720;

  let svg;
  let simulation;

  onMount(() => {

    const linkData = graphData.links.map(link => ({
      ...link,
      source: graphData.nodes.find(node => node.id === link.source),
      target: graphData.nodes.find(node => node.id === link.target),
    }));
    // D3.js code here (same as in your previous code)

    // Your D3.js code here
    svg = d3.select('#graph')
      .append('svg')
      .attr('width', width)
      .attr('height', height);

    simulation = d3.forceSimulation()
      .force('link', d3.forceLink().id(d => d.id))
      .force('charge', d3.forceManyBody().strength(-120))
      .force('center', d3.forceCenter(width / 2, height / 2));

    const link = svg.append('g')
      .selectAll('line')
      .data(linkData) // Use the updated linkData array
      .enter()
      .append('line')
      .style('stroke', '#464F51')
      .style('stroke-width', 2);

    const node = svg.append('g')
      .selectAll('circle')
      .data(graphData.nodes)
      .enter()
      .append('circle')
      .attr('r', 10)
      .style('fill', '#724cf9')
      .call(d3.drag()
        .on('start', dragStarted)
        .on('drag', dragging)
        .on('end', dragEnded)
      );

    node.append('title')
      .text(d => d.name);

    simulation.nodes(graphData.nodes).on('tick', ticked);
    simulation.force('link').links(linkData); // Use the updated linkData array

    function ticked() {
      link
        .attr('x1', d => d.source.x)
        .attr('y1', d => d.source.y)
        .attr('x2', d => d.target.x)
        .attr('y2', d => d.target.y);

      node
        .attr('cx', d => d.x)
        .attr('cy', d => d.y);
    }

    function dragStarted(event, d) {
      if (!event.active) simulation.alphaTarget(0.3).restart();
      d.fx = d.x;
      d.fy = d.y;
    }

    function dragging(event, d) {
      d.fx = event.x;
      d.fy = event.y;
    }

    function dragEnded(event, d) {
      if (!event.active) simulation.alphaTarget(0);
      d.fx = null;
      d.fy = null;
    }

    function handleResize() {
      width = window.innerWidth;
      height = window.innerHeight;

      // Update SVG width and height
      svg.attr('width', width).attr('height', height);

      // Update force center
      simulation.force('center', d3.forceCenter(width / 2, height / 2));

      // Restart the simulation to update the positions of nodes and links
      simulation.alpha(1).restart();
    }

    // Add window resize event listener
    window.addEventListener('resize', handleResize);
  });
</script>

<div id="graph">
  <!-- The graph will be rendered inside this div -->
</div>
