# rs-flow
System of flows in rust

You can see examples in '/crates/examples' folder.

## Next feautures
<ul>
  <li>[x] Update errors </li>
  <li>[x] Run flow async </li>
  <li>[x] Add global data to the flow, which can be accessed by components</li>
  <li>[x] Return Global when finish flow run, without cloned</li>
  <li>[x] Refector Inputs and Outputs ports and Component struct and trait </li>
  <li>[x] Turn a workspace</li>
  <li>[x] Macros to implement Inputs and Outputs trait by component</li>
  <li>[x] Update for a queue by component, em merge queues at cicle</li>
  <li>[x] Check if a connection create a Loop </li>
  <li>[x] Components can return { Continue or Break } that dermine if a flow continue or stop your execution</li>
  <li>[x] Create component types { Lazy or Eager } that define when a component will be executed </li> 
  <li>[x] Update Packges for include Bytes type</li>
  <li>[ ] Check if a package has been consumed from queue (Loop detected) </li>
  <li>[ ] Run flow components in parallel </li>
  <li>[ ] Docs </li>
</ul>
