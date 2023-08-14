<script>
    import * as D3vars from "./D3vars.js";
    import { onMount } from "svelte";
    import * as d3 from "d3";

    export let width;
    export let height;
    export let graphData;
    export let connection_cache;
    export let parsed_data;

    let linkDistance = 100;
    let nodeStrength = -10;

    const simulation = D3vars.setupSimulation(width, height, nodeStrength, linkDistance);

    const updateGraph = () => {
        simulation.force("link").distance(linkDistance);
        simulation.alpha(1).restart();
        // Recenter the nodes
        simulation
            .force("charge", d3.forceManyBody().strength(nodeStrength))
            .force("center", d3.forceCenter(width / 2, height / 2));
    };

    onMount(() => {
        const linkData = graphData.links.map((link) => ({
            ...link,
            source: graphData.nodes.find((node) => node.id === link.source),
            target: graphData.nodes.find((node) => node.id === link.target),
        }));
        // D3.js code here (same as in your previous code)

        // Your D3.js code here
        const svg = D3vars.setupSVG(width, height);

        const link = D3vars.createLinks(svg, linkData, parsed_data);

        const tooltip = D3vars.createTooltip();

        const node = D3vars.createNodes(svg, graphData)
            .call(
                d3
                    .drag()
                    .on("start", dragStarted)
                    .on("drag", dragging)
                    .on("end", dragEnded)
            )
            .on("mousemove", handleMouseMove)
            .on("mouseout", handleMouseOut);

        function handleMouseMove(event, d) {
            const [mouseX, mouseY] = d3.pointer(event);
            tooltip
                .style("opacity", 0.9)
                .html(d.id) // Use the node's name from data
                .style("left", mouseX + 10 + "px")
                .style("top", mouseY - 20 + "px");

            updateNodeStyling(d.id);
        }

        function handleMouseOut() {
            tooltip.style("opacity", 0);
            updateNodeStyling(null);
        }

        async function updateNodeStyling(clickedID) {
            node.style("fill", (d) => {
                if (clickedID === null) return "#724cf9";
                if (!connection_cache.has(clickedID)) return "#724cf9";
                const cacheValue = Array(connection_cache.get(clickedID));
                if (cacheValue.length == 0) return "#724cf9";
                // I FUCKING HATE JAVASCRIPT AAAHHH!
                const isIdInCache = cacheValue.at(0).includes(d.id);
                if (isIdInCache) {
                    console.log("Changed!");
                    return "red";
                } else {
                    return "#724cf9";
                }
            });
        }

        simulation.nodes(graphData.nodes).on("tick", ticked);
        simulation.force("link").links(linkData); // Use the updated linkData array

        function ticked() {
            link.attr("x1", (d) => d.source.x)
                .attr("y1", (d) => d.source.y)
                .attr("x2", (d) => d.target.x)
                .attr("y2", (d) => d.target.y);

            // Constrain node positions within the SVG boundaries
            node.attr("cx", (d) => {
                return (d.x = Math.max(0, Math.min(width, d.x)));
            }).attr("cy", (d) => {
                return (d.y = Math.max(0, Math.min(height, d.y)));
            });

            // updateNodeStyling();
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
    <div class="slider-container">
        <input
            type="range"
            min="1"
            max="500"
            step="1"
            bind:value={linkDistance}
            class="slider"
            on:input={updateGraph}
        />
        <span class="slider-label">Link Distance: {linkDistance}</span>
        <input
            type="range"
            min="-300"
            max="100"
            step="1"
            bind:value={nodeStrength}
            class="slider"
            on:input={updateGraph}
        />
        <span class="slider-label">Strength: {nodeStrength}</span>
    </div>
</div>
