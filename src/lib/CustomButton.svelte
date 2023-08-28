<script>
    export let label = "Button";
    export let bgColor = "none";
    export let txColor = "none";
    export let border = "none";
    export let borderColor = "none";
    export let isToggle = "false";
    export let toggleColor = "none";
    export let onClick = () => {}; // Default click handler
    export let onToggle = (value) => {}
    
    const initalStyle = `background-color: ${bgColor}; border: ${border} ${borderColor}; color: ${txColor};`;
    let buttonStyle = initalStyle;
    let isActive = false;

    const handleCick = () => {
      console.log(isToggle);
      if((isToggle === "true" || isToggle === "True") && !isActive) {
        buttonStyle = `background-color: ${bgColor}; border: ${border} ${toggleColor}; color: ${toggleColor};`
        isActive = true;
        onToggle(true);
      } else if (isActive) {
        isActive = false;
        onToggle(false);
        buttonStyle = initalStyle;
      } else {
        onClick();
      }
    }

  </script>
  
  <button
    class="custom-button"
    style="{buttonStyle}"
    on:click={handleCick}
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
  