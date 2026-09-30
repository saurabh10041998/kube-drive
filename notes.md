## Decisions on the fly

Run the service on the jumphost which which starts the API server 
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
| `/pods/list?ns=<namespace>` | GET | list the pods which are present in namespace |
| `/namespace/list` | GET | list the namespace |
| `/pods/<pod-name>/lsr` | GET | List the root file / folders (depth 1) for pod `<pod-name>` |
| `/pods/<pod-name>/ls` <br> <pre> ``` { "parent": ... }``` </pre> | POST | List the folder in path suggested by `parent`  |
