<script>
    export let label = "Button";
    export let color = "blue";
    export let onClick = () => {}; // Default click handler
  </script>
  
  <button
    class="custom-button"
    style="background-color: {color};"
    on:click={onClick}
  >
    {label}
  </button>
  
  <style>
    .custom-button {
      padding: 10px 20px;
      border: none;
      color: white;
      cursor: pointer;
      border-radius: 5px;
      transition: background-color 0.2s ease;
    }
  </style>
  