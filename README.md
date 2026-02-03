# workflows-rs
A rust wrapper for [cloudflare workflows](https://www.cloudflare.com/developer-platform/products/workflows/)

> [!WARNING]  
> This will probably be obsolete soon ([#918](https://github.com/cloudflare/workers-rs/pull/918))

## Usage
1. Add this repo to your Cargo.toml:
    `workflows-rs = { git = "https://github.com/Sushi-Mampfer/workflows-rs" }`
2. Modify your build command to run `workflows-build` after `worker-build`. You can find the source [here](https://github.com/Sushi-Mampfer/workflows-build).
3. Create your workflow and add it to wrangler.toml: \
    Most types can be replaced with types that implement Serialize and Deserialize(the compiler will tell you if it's missing something). This includes the return type.
```rust
#[wasm_bindgen]
pub struct TestWorkflow {
    env: Env,
}

#[wasm_bindgen]
impl TestWorkflow {
    #[wasm_bindgen(constructor)]
    pub fn new(_ctx: JsValue, env: Env) -> Self {
        Self { env: env }
    }

    pub async fn run(&self, event: JsValue, step: WorkflowStep) -> Result<String, JsValue> {
        let event: WorkflowEvent<String> = from_value(event).unwrap();
        Ok(event.payload)
    }
}
```
```toml
[[workflows]]
name = "test-workflow"
binding = "TESTWORKFLOW"
class_name = "TestWorkflow"
```

4. Use is like you would use js workflows(more or less):
```rust
#[event(fetch)]
async fn fetch(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    if req.path() == "/" {
        Response::ok(
            env.workflow("TESTWORKFLOW")
                .unwrap()
                .create::<String>(Some(WorkflowInstanceCreateOptions::new(
                    None,
                    Some("Hello world".to_string()),
                )))
                .await
                .unwrap()
                .id(),
        )
    } else {
        let mut id = req.path();
        id.remove(0);
        let instance = env.workflow("TESTWORKFLOW").unwrap().get(id).await.unwrap();
        let status = instance.status::<String>().await.unwrap();
        match status.status {
            workflows_rs::InstanceStatuses::Running => Response::ok("running")
            workflows_rs::InstanceStatuses::Complete => Response::ok(status.output.unwrap()),
            _ => Response::ok("Something went wrong"),
        }
    }
}
```
### A note on step.do
`do` is a reserved keywork in rust and therfore I have renamed it to `exec`. \
In js you can pass a closure that can be async but doesn't have to be. In rust this is (to my knowledge) not possible. \
Because most of workers-rs relies on async exec takes a Fn() that returns a future, this can be done with `|| async {}` \
For more complicated functions that need `env` you have to clone it in the outer, sync closure:
```rust
let env = self.env.clone();
let bucket_content = step
    .exec("test-step", None, move || {
        let env = env.clone();
        async move {
            Ok(env
                .bucket("TEST")
                .unwrap()
                .get("test")
                .execute()
                .await
                .unwrap()
                .unwrap()
                .body()
                .unwrap()
                .text()
                .await
                .unwrap()
            )
        }
    })
        .await
        .unwrap();
```
### A note on js errors
If your future returns an `Err` it's treades as a js error with the same content.

## TODO
- [ ] Macros for more safety
- [ ] Somehow removing the need for `workflows-build`
- [ ] More constructors for config structs 

Except for the last one I currently lack the knowledge to do it, and with there being a pr that would add the same functionality to the official workers-rs repo I don't see much of a reason to learn it.