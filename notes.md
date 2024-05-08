# Waker
A Waker is a handle for **waking up a task by notifying its executor that it is ready to be run**.

This handle encapsulates a RawWaker instance, which defines the executor-specific wakeup behavior.

The typical life of a Waker is that it is **constructed by an executor**, wrapped in a Context, then passed to Future::poll(). Then, if the future chooses to return Poll::Pending, **it must also store the waker somehow and call Waker::wake() when the future should be polled again**.

Implements Clone, Send, and Sync; therefore, a waker may be invoked from any thread, including ones not in any way managed by the executor. For example, this might be done to wake a future when a blocking function call completes on another thread.

Note that it is preferable to use waker.clone_from(&new_waker) instead of *waker = new_waker.clone(), as the former will avoid cloning the waker unnecessarily if the two wakers wake the same task.

Constructing a Waker from a RawWaker is unsafe. Implementing the Wake trait is a safe alternative that requires memory allocation.

## wake(self)
Wake up the task associated with this Waker.

As long as the executor keeps running and the task is not finished, it is guaranteed that each invocation of wake() (or wake_by_ref()) will be followed by at least one poll() of the task to which this Waker belongs.


## wake_by_ref(&self)

This is similar to wake(), but may be slightly less efficient in the case where an owned Waker is available. This method should be preferred to calling waker.clone().wake().