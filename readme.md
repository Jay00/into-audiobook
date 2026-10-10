# Running the Container

## Locally Available Directory

MSYS_NO_PATHCONV=1 docker run --gpus all \
 -v C:/Users/jason/Downloads/test:/app/data \
 text-to-audio

## Docker SMB / Network Drive Setup

Due to Windows user-session limitations, mapped network drives (e.g., `R:\`) are not visible to the Docker daemon. Use a Docker CIFS volume to attach directly to the SMB share.

### 1. Create the Docker SMB Volume (One-Time Setup)

Run in terminal (adjust IP, share name, and credentials as needed):

```bash
docker volume create \
  --driver local \
  --opt type=cifs \
  --opt device="//10.10.10.2/test" \
  --opt o="username=guest,password=,vers=3.0" \
  research_smb

MSYS_NO_PATHCONV=1 docker run --gpus all \
  --mount "type=volume,src=research_smb,dst=/app/data,volume-subpath=test" \
  text-to-audio
```
