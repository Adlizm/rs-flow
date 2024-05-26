# rs-flow
System of flows in rust

You can see a example in '/src' folder.

## Next feautures
<ul>
  <li>[x] Update errors </li>
  <li>[x] Run flow async </li>
  <li>[x] Add global data to the flow, which can be accessed by components</li>
  <li>[x] Return Global when finish flow run, without cloned</li>
  <li>[x] Refector Inputs and Outputs ports and Component struct and trait </li>
  <li>[x] Turn a workspace</li>
  <li>[x] Macros to implement Inputs and Outputs trait by component</li>
  <li>[ ] Run flow components in parallel </li>
  <li>[ ] Benchmark for queue implementations:
    <ul>
      <li>[ ] Unique queue controller with DashMap</li>
      <li>[ ] Unique queue controller with Simple HashMap with key Mutex</li>
      <li>[ ] Single queue by component, em merge queues at cicle</li>
    </ul>
  <li>[ ] Check if a package has been consumed from queue (Loop detected) </li>
</ul>
