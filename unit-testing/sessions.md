# Sessions

## Session 1: Unit Tests

Session summary:

- When writing tests we want to verify the properties of the code we’re
  testing, not the implementation (example: addition, commutativity and associativity)

- Instead of hardcoding specific input values, we can use a property-based
  testing tool like Quickcheck or Hypothesis to generate values for us. This
  has the advantage of covering cases we did not think of, and providing broader
  coverage than tables of values can buy us.

- When using such a tool, we can define the ranges of values that we want to
  test (by using the language’s type system or by using a custom API defined by
  the tool). Carefully defining the relevant ranges for the code (e.g. all valid
  day/month/year values for a date parser) will allow for effective coverage.

- The tools provide a shrinking mechanism that tries to reduce input for failed
  tests to the simplest and smallest values (e.g. small numbers, short strings)
  that still cause failure. This makes it much easier to find the bug.

- Some tools allow saving output for failed tests automatically to allow easy verification of regressions.

- Introducing a random factor into tests is a good thing, since it greatly
  improves coverage. It is important to store the seed so that the test can be
  rerun later with the exact same values.

