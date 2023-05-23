# age-test

There's no option to set the work factor with password-based encryption (scrypt) using the `rage` library and the default uses a lot of memory:

![heaptrack](https://user-images.githubusercontent.com/191720/235452993-64a18d40-9625-4f7e-963a-9450c210c7b7.png)

See [this issue](https://github.com/str4d/rage/issues/383).

