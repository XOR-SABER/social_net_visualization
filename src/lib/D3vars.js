import * as d3 from "d3";

export function setupSVG(width, height) {
    return d3
        .select("#graph")
        .append("svg")
        .attr("width", width)
        .attr("height", height);
}

export function setupSimulation(width, height, nodeStrength, linkDistance) {
    // ...
    return d3
        .forceSimulation()
        .force(
            "link",
            d3
                .forceLink()
                // @ts-ignore
                .id((d) => d.id)
                .distance(linkDistance)
        )
        .force("charge", d3.forceManyBody().strength(nodeStrength))
        .force("center", d3.forceCenter(width / 2, height / 2));
}

export function createLinks(svg, linkData, parsed_data) {
    // ...
    svg.append("defs")
        .selectAll("marker")
        .data(["arrow"])
        .enter()
        .append("marker")
        .attr("id", (d) => d)
        .attr("viewBox", "0 -5 10 10")
        .attr("refX", 30)
        .attr("refY", 0)
        .attr("markerWidth", 6)
        .attr("markerHeight", 6)
        .attr("orient", "auto")
        .append("path")
        .attr("d", "M0,-5L10,0L0,5")
        .attr("fill", "#737a86");

    return svg
        .append("g")
        .selectAll("line")
        .data(linkData)
        .enter()
        .append("line")
        .style("stroke", "#464F51")
        .style("stroke-width", 2)
        .attr("marker-end", (d) => {
            const isMutual =
                parsed_data.LinkElms[
                    parsed_data.NodeElms.indexOf(d.source.id)
                ].includes(d.target.id) &&
                parsed_data.LinkElms[
                    parsed_data.NodeElms.indexOf(d.target.id)
                ].includes(d.source.id);
            return isMutual ? "" : "url(#arrow)";
        });
}

export function createTooltip() {
    return d3
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
}

export function createNodes(svg, graphData) {
    // ...
    return svg
        .append("g")
        .selectAll("circle")
        .data(graphData.nodes)
        .enter()
        .append("circle")
        .attr("r", 10)
        .style("fill", "#724cf9");
}