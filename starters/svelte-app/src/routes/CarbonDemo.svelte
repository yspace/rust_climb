<script>
	import { ButtonSet, Button } from "carbon-components-svelte";
	// import Login16 from "carbon-icons-svelte/lib/Login16";
    import { Accordion, AccordionItem } from "carbon-components-svelte";
</script>

<svelte:head><link rel="stylesheet" href="https://unpkg.com/carbon-components-svelte@0.30.0/css/g10.css" /></svelte:head>

<div style="padding: 2rem;">
	<ButtonSet>
		
		<Button kind="primary">Save</Button>
	</ButtonSet>
</div>


 
  
  <Accordion>
	<AccordionItem title="Section 1" open> Content 1 </AccordionItem>
	<AccordionItem title="Section 2"> Content 2 </AccordionItem>
	<AccordionItem title="Section 3"> Content 3 </AccordionItem>
  </Accordion>