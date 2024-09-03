## WLT Program Flow

1. Collect Input for Load Test
   LT: Declarative Files

- HTTP Request(s)

  - Headers
  - Content

- Test Parameters

  - Threads
  - LT: Multiple Transactions
  - LT: Timeouts
  - LT: Retries on failed attempts
  - LT: Ramp up time

2. Initialize the state from the Input

- properly handle the dynamic configuration

3. Run the test

4. Collect test information

- avg, min, max of requests

  - for each request in a series
  - for the series if multiple requests
  - for the test

- time to respond over the duration of the load test

- failures
  - connection
  - errors

5. Return aggragated data

- Terminal Formatted
- File Output
  - Templating for HTML?
- LT: GUI and graphs

_\*LT is long term solutions that may not be implemented at the time of viewing._
