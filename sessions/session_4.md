# Let's Rust!

## Session 4

#### Table of contents
[WIP]

### Goals
- [x] Use the model and the client sent features to generate and return a prediction.
- [ ] Dockerize the API
- [ ] Dockerize the training script.


### What to do when you get an `no space left on device` error in Docker

#### Option 1 -> Free up some space (start with this)
```bash
docker system df

TYPE            TOTAL     ACTIVE    SIZE      RECLAIMABLE
Images          54        38        28.41GB   12.75GB (44%)
Containers      43        2         1.069GB   1.069GB (99%)
Local Volumes   13        3         462.6MB   415.7MB (89%)
Build Cache     172       0         20.75GB   20.75GB
```

In my case, the build cache is huge. Let's drop it:
```
docker builder prune -f

Total:  23.96GB
```

```bash
docker system df

TYPE            TOTAL     ACTIVE    SIZE      RECLAIMABLE
Images          43        38        21.56GB   5.898GB (27%)
Containers      43        2         1.069GB   1.069GB (99%)
Local Volumes   13        3         462.6MB   415.7MB (89%)
Build Cache     95        0         0B        0B
```

#### Option 2 -> Increase disk space for Docker
If you find yourself constantly freeing space for your docker engine, you better
give your docker engine a bit more of your disk space.

1. Open Docker Desktop
2. Click on the gear icon (⚙️) to open Settings
3. Go to "Resources" → "Advanced"
4. Adjust the slider for "Disk image size" (default is usually 64GB)
5. Click "Apply & Restart"




### Challenges

#### Hyperparameter tuning in Rust
Olanrewaju asked
```
How can we cross-validate?
```
It seems there is a library to do hyperparameter tuning in Rust.
It is called [hyperopt-rs](https://docs.rs/hyperopt/latest/hyperopt/)

Can you add hyperparameter tuning into the training?

#### Try another ML model
As Tiamiyu pointed out, there is another ML library called [linfa](https://docs.rs/linfa/latest/linfa/)

Can you train another ML model for this dataset?


#### Dockerize the training script
Use a multi-stage build, as we did for the API.