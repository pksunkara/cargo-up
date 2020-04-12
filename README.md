# cargo-up

Upgrade your dependencies by automatically fixing your code

## Workflow

Assuming we have a project with the following `Cargo.toml`

```toml
[dependencies]
foo = "0.8.2"
```

We can use the upgrade workflow in our project as shown below:

```
+----------------------+        +--------------------------------+
|                      |        |                                |
|  foo@2.0.0 released  |        | Run all foo_up from old to new |
|                      |        |                                |
+----------+-----------+        +----------------+---------------+
           |                                     ^
           |                                     |
           |                                     |
           |                                     |
           v                                     |
  +--------+---------+            +--------------+-------------+
  |                  |            |                            |
  |   cargo up foo   +----------->+ Download all foo_up@>0.8.2 |
  |                  |            |                            |
  +------------------+            +----------------------------+
```

## Contributors
Here is a list of [Contributors](http://github.com/pksunkara/cargo-up/contributors)

### TODO

__I accept pull requests__

## License
MIT/X11

## Bug Reports
Report [here](http://github.com/pksunkara/cargo-up/issues).

## Contact
Pavan Kumar Sunkara (pavan.sss1991@gmail.com)

Follow me on [github](https://github.com/users/follow?target=pksunkara), [twitter](http://twitter.com/pksunkara)
