# 1.1
- Added ```alloc``` and ```macro ``` features
- Added ```prelude``` module
- Added ```Boo``` type alias
- Implemented ```AsRef``` and ```AsMut```
- Deprecated ```some_left``` and ```some_right``` (renamed to ```left``` and ```right```)
- Added ```left_unchecked``` and ```right_unchecked```
- Added ```EitherIter``` trait
- Extended documentation

# 1.1.2
- Added ```LRIter```
- Implemented ```Iterator```, ```ExactSizeIterator```, ```DoubleSizedIterator``` and ```FusedIterator```
- Added ```size_hint``` to ```EitherIter``` iterators
- Added ```as_inner_ref``` to ```Boo```

# 1.1.3
-  Added `ok_left` and `ok_right`

# 1.1.4
- Removed `Unpin` requirement to implement `Future` and `Stream`

# Roadmap
- Add ```EitherStream```