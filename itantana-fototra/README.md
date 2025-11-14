# How to add an adapter into the runtime

Adapter loading is always before Runtime initialization.

1. Implement the trait ``` AdapterLoaderTrait ``` into your Adapter
2. Call ``` Adapter::insert(Box::new(`Your adapter here`)).await; ```

# How to initialize the runtime

Just call the function ``` Runtime::init().await.unwrap(); ```

# How to get the configuration from the configuration file

Just do like this ``` let configuration: Option<Arc<Configuration>> = Runtime::get().await; ```

## How to get the configuration value you just add

Just do like this example ``` let bar = configuration.get("bar"); ```

## How to get a secret value

The **config.toml** and the **secret.toml** are merged automatically after there are loaded. So you can do like this ``` let secret = configuration.get("secret"); ```

# How to add data inside the runtime so that you can get it everywhere

After the Runtime is initialized, you can call everywhere in the program this function ``` Runtime::register(`your variable`).await; ```.

Each time you register a data inside the runtime, you update the value of it.

# How to get data from the runtime from everywhere

You can just load data inside the runtime from everywhere like this ``` Runtime::get::<`your data type`>().await; ```.
