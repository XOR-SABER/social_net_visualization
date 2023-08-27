<script>
    export let label = "Button";
    export let bgcolor = "none";
    export let txcolor = "none";
    export let border = "none";
    export let isToggle = "false";
    export let onClick = () => {}; // Default click handler
    
    let buttonStyle = "";

    const handleCick = () => {
      if(isToggle === "true" || isToggle === "True") {
        
      } else {

      }

    }

  </script>
  
  <button
    class="custom-button"
    style="background-color: {bgcolor}; border: {border}; color: {txcolor};"
    on:click={onClick}
  >
    {label}
  </button>
  
  <style>
    .custom-button {
      padding: 10px 20px;
      border: none;
      cursor: pointer;
      border-radius: 2px;
    }
  </style>
  