<script>
    import * as D3vars from "./D3vars.js";
    import CustomButton from "./CustomButton.svelte";
    import CustomSlider from "./CustomSlider.svelte";
    import { onMount } from "svelte";
    import * as d3 from "d3";
    import { invoke } from "@tauri-apps/api/tauri";

    export let width;
    export let height;
    export let graphData;
    export let parsed_data;

    let linkDistance = 100;
    let nodeStrength = -10;
    let NonOpacity = 0.4;
    let selectedNode = "";
    let tooltip;
    let link;
    let node;
    let svg;

    const simulation = D3vars.setupSimulation(
        width,
        height,
        nodeStrength,
        linkDistance
    );

    const updateGraph = () => {
        simulation.force("link").distance(linkDistance);
        simulation.alpha(0.1).restart();
        // Recenter the nodes
        simulation
            .force("charge", d3.forceManyBody().strength(nodeStrength))
            .force("center", d3.forceCenter(width / 2, height / 2));
    };

    const handleConnections = (value) => {
        console.log("Handler Connections");
        if (value) connectionNodeStyling();
        else resetNodeStyling();
    };

    const handleDfs = (value) => {
        console.log("Handler DFS");
        if (value) dfsNodeStyling();
        else resetNodeStyling();
    };

    const handleBfs = (value) => {
        console.log("Handler BFS");
        if (value) bfsNodeStyling();
        else resetNodeStyling();
    };

    async function bfsNodeStyling() {
        const conncections = await invoke("send_bfs", { id: selectedNode });
        console.log(conncections);
        let colorIndex = 0;
        node.style("fill", (d) => {
            const HSLStr = "HSL(" + colorIndex + ", 100%, 50%";
            if (d.id === selectedNode) return "green";
            if (conncections.includes(d.id)) {
                colorIndex += 20;
                return HSLStr;
            } else return "#724cf9";
        });
    }

    async function dfsNodeStyling() {
        const conncections = await invoke("send_dfs", { id: selectedNode });
        console.log(conncections);
        let colorIndex = 0;
        node.style("fill", (d) => {
            const HSLStr = "HSL(" + colorIndex + ", 100%, 50%";
            if (d.id === selectedNode) return "green";
            if (conncections.includes(d.id)) {
                colorIndex += 20;
                return HSLStr;
            } else return "#724cf9";
        });
    }

    async function connectionNodeStyling() {
        const connections = await invoke("send_graph_connections", {
            id: selectedNode,
        });
        console.log(connections);
        node.style("fill", (d) => {
            if (d.id === selectedNode) return "green";
            if (connections.includes(d.id)) {
                return "red";
            } else return "#724cf9";
        }).style("opacity", (d) => {
            if (d.id === selectedNode) return 1;
            if (connections.includes(d.id)) {
                return 1;
            } else return NonOpacity;
        });
        link.style("stroke", (d) => {
            // Check if the source and target of the link are in connections
            if (d.source.id == selectedNode) {
                return "white"; // Change to the desired lighter color
            }
            return "#464F51"; // Default color for links
        })
            .style("opacity", (d) => {
                if (!(d.source.id == selectedNode)) {
                    return NonOpacity; // Adjust the opacity for non-connected links
                }
                return 1; // Default opacity for connected links
            })
            .attr("marker-end", (d) => {
                if (d.source.id == selectedNode) {
                    return "url(#arrow-white)";
                }
                return null; // Remove marker for connected links
            });
    }

    async function resetNodeStyling(clickedID) {
        node.style("fill", (d) => {
            if (d.id === selectedNode) return "green";
            return "#724cf9";
        }).style("opacity", (d) => {
            return 1; // Default opacity for connected links
        });
        link.style("stroke", (d) => {
            // Check if the source and target of the link are in connections
            return "#464F51"; // Default color for links
        })
            .style("opacity", (d) => {
                return 1; // Default opacity for connected links
            })
            .attr("marker-end", (d) => {
                // if (d.source.id == selectedNode) {
                //     return "url(#arrow)";
                // }
                return "url(#arrow)"; // Remove marker for connected links
            });
    }

    function init() {
        const linkData = graphData.links.map((link) => ({
            ...link,
            source: graphData.nodes.find((node) => node.id === link.source),
            target: graphData.nodes.find((node) => node.id === link.target),
        }));
        // D3.js code here (same as in your previous code)

        // Your D3.js code here
        svg = D3vars.setupSVG(width, height);

        link = D3vars.createLinks(svg, linkData, parsed_data);

        tooltip = D3vars.createTooltip();

        node = D3vars.createNodes(svg, graphData)
            .call(
                d3
                    .drag()
                    .on("start", dragStarted)
                    .on("drag", dragging)
                    .on("end", dragEnded)
            )
            .on("mousemove", handleMouseMove)
            .on("mouseout", handleMouseOut)
            .on("click", (event, d) => {
                handleNodeClick(event, d);
            });

        function handleMouseMove(event, d) {
            const [mouseX, mouseY] = d3.pointer(event);
            tooltip
                .style("opacity", 0.9)
                .html(d.id) // Use the node's name from data
                .style("left", mouseX + 10 + "px")
                .style("top", mouseY - 20 + "px");

            // resetNodeStyling(d.id);
        }

        function handleNodeClick(event, node) {
            event.stopPropagation();
            // Implement your logic to display options for the clicked node
            console.log("Node clicked:", node.id);
            selectedNode = node.id;
            resetNodeStyling();
            // You can show a modal or update a state to show options
        }

        function handleMouseOut() {
            tooltip.style("opacity", 0);
            // resetNodeStyling(null);
        }

        simulation.alpha(0.1).restart();
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
            // resetNodeStyling();
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
            if (!event.active) simulation.alphaTarget(0.1).restart();
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
            simulation.alpha(0.1).restart();
        }

        // Add window resize event listener
        window.addEventListener("resize", handleResize);
    }

    onMount(() => {
        init();
    });
</script>

<div id="graph">
    <div class="options_container">
        {#if selectedNode}
            <p>Current node selected: {selectedNode}</p>
            <CustomButton
                label="DFS"
                border="solid 2px"
                borderColor="white"
                bgColor="black"
                txColor="white"
                isToggle="true"
                toggleColor="Green"
                onToggle={handleDfs}
            />
            <CustomButton
                label="BFS"
                border="solid 2px"
                borderColor="white"
                bgColor="black"
                txColor="white"
                isToggle="true"
                toggleColor="Green"
                onToggle={handleBfs}
            />
            <CustomButton
                label="Connections"
                border="solid 2px"
                borderColor="white"
                bgColor="black"
                txColor="white"
                isToggle="true"
                toggleColor="Green"
                onToggle={handleConnections}
            />
        {/if}
    </div>
    <div class="slider_container">
        <CustomSlider
            min={0}
            max={750}
            step={1}
            label="Link Distance"
            onMove={updateGraph}
            bind:value={linkDistance}
        />
        <CustomSlider
            min={-300}
            max={300}
            step={1}
            label="Node Strength"
            onMove={updateGraph}
            bind:value={nodeStrength}
        />
        <CustomSlider
            min={0.0}
            max={1}
            step={0.01}
            label="Unrelated node Opacity"
            onMove={updateGraph}
            bind:value={NonOpacity}
        />
    </div>
</div>

<style>
    .options_container {
        position: absolute;
        margin-left: auto;
        margin-right: auto;
        left: 0;
        right: 0;
        text-align: center;
    }
    .slider_container {
        position: absolute;
        top: 20px;
        right: 20px;
        display: flex;
        flex-direction: column;
        align-items: flex-end;
    }
</style>
