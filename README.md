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
