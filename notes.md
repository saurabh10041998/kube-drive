## Decisions on the fly

Run the service on the jumphost which starts the API server 
On the jumphost 
And user interacts it via web API.  

## backend (thoughts)
Kube-drive: Google drive for the kubernetes pods

I want to develop web api server (Rust) which will interact with container file system (Note: there are multiple container in the pod) using API. 

I will start this as service (Most likely as systemd service) on host which has access to kubernetes cluster. But at start it can be standalone binary like this
```bash
./kube-drive <option> [-f config-file]
```

### Backend API design
These are possible API routes I have thought of

| API Route | Type | Description |
|-----------|------|-------------|
| `/api/v1/namespaces/{ns}/pods` | GET | list the pods which are present in namespace |
| `/api/v1/namespaces` | GET | list the namespace |
| `/api/v1/namespaces/{ns}/pods/{pod}` | GET | pod details {container, status} |
| `/api/v1/namespaces/{ns}/pods/{pod}/containers/{container}/fs?path=/var/log&depth=1` | POST | List the folder in path suggested by `path` upto `depth` 1 |


## Text editor 
sublime text

## Tech stack
- Backend: Rust
- Frontend: Angular

## Backend libraries
- Web api design - Axum
- Frontend - Angular
- Monitoring - Prometheus 

### I want to use these technology but first want to evaluate whether these fit in any of this project idea
- Graphql
- Elasticsearch