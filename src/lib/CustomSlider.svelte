<script>
    export let min = 0;
    export let max = 0;
    export let step = 0;
    export let value = 0;
    export let label = '';
    export let onMove = () => {}; // Default click handler
    $:value;
</script>

<span class="slider-label">{label}: {value}</span>
<input
type="range"
min={min}
max={max}
step={step}
bind:value
class="slider"
on:input={onMove}/>

<style>
    .slider {
    width: 100px;
    margin-top: 5px;
    }

    .slider-label {
    font-size: 14px;
    margin-top: 5px;
    }
</style>