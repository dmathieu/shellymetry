# ShellyMetry

Fetch data from [shelly.cloud](https://shelly.cloud/) devices, and expose it as
Prometheus metrics.

## Usage

For now, you need to pull the repository locally, and build it.

With rust installed:

```
cargo install shellymetry
```

You then need a `shellymetry.json` file with the following content:

```
{
	"server_port": 1304,
	"refresh_interval": 15,
	"devices": [
		{
			"kind": "plug",
			"name": "D9CFDD",
			"labels": {
				"name": "office"
			}
		}
	],
	"otlp_endpoint": "<endpoint>", // Optional OTLP endpoint to send traces to
	"otlp_headers": { } // Optional OTLP headers to send with the traces (for authentication)
}
```

If OTLP isn't configured, traces will be sent to stdout.

Run the process:

```
shellymetry -c shellymetry.json
```
