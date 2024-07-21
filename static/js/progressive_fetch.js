///https://github.com/bevyengine/bevy-website/issues/338
async function progressive_fetch(resource, callbacks={}) {
    // Allow users to specify only the callbacks they need.
    const cb = Object.assign({
      start: (length) => {},
      progress: (progress, length) => {},
      flush: (length) => {},
    }, callbacks);
    let response
    try {
        response = await fetch(resource);
    } catch(e) {
        console.error(e)
    }
  
    // get the length and initiallise progress to 0.
    // const length = response.headers.get('content-length');
    const length = response.headers.get('X-Original-Content-Length');
    let prog = 0;
  
    const transform = new TransformStream({
      start() {
        // When the Stream is first created, call the user-specified "start"
        // callback to do any setup required.
        cb.start(length);
      },
      transform(chunk, controller) {
        // See how much data has been loaded since we last checked, then call
        // the user-specified "progress" callback with the current progress &
        // total length.
        prog += chunk.byteLength;
        cb.progress(prog, length);
        // Simply pass through the data without touching it.
        controller.enqueue(chunk);
      },
      flush() {
        // When the Stream has finished, call the user-specified "finish" callback
        // to do any cleanup necessary.
        cb.flush(length);
      },
    });
  
    // Give the caller a new version of the Response where we pass its
    // ReadableStream through the user's TransformStream.
    return new Response(response.body.pipeThrough(transform), response)
  }