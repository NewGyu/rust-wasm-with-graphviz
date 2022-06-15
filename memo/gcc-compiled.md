The case when without `CMAKE_TOOLCHAIN_FILE`.

```
vscode ➜ ~/clib-example/clib-graphviz (master) $ cargo build
   Compiling clib-graphviz v0.1.0 (/home/vscode/clib-example/clib-graphviz)
error: failed to run custom build command for `clib-graphviz v0.1.0 (/home/vscode/clib-example/clib-graphviz)`

Caused by:
  process didn't exit successfully: `/home/vscode/clib-example/target/debug/build/clib-graphviz-af4fec6f5c04fc98/build-script-build` (exit status: 101)
  --- stdout
  CMAKE_TOOLCHAIN_FILE_x86_64-unknown-linux-gnu = None
  CMAKE_TOOLCHAIN_FILE_x86_64_unknown_linux_gnu = None
  HOST_CMAKE_TOOLCHAIN_FILE = None
  CMAKE_TOOLCHAIN_FILE = None
  CMAKE_GENERATOR_x86_64-unknown-linux-gnu = None
  CMAKE_GENERATOR_x86_64_unknown_linux_gnu = None
  HOST_CMAKE_GENERATOR = None
  CMAKE_GENERATOR = None
  CMAKE_PREFIX_PATH_x86_64-unknown-linux-gnu = None
  CMAKE_PREFIX_PATH_x86_64_unknown_linux_gnu = None
  HOST_CMAKE_PREFIX_PATH = None
  CMAKE_PREFIX_PATH = None
  CMAKE_x86_64-unknown-linux-gnu = None
  CMAKE_x86_64_unknown_linux_gnu = None
  HOST_CMAKE = None
  CMAKE = None
  running: "cmake" "/home/vscode/clib-example/clib-graphviz/cpp" "-DCMAKE_BUILD_TYPE=MinSizeRel" "-DCMAKE_INSTALL_PREFIX=/home/vscode/clib-example/target/debug/build/clib-graphviz-8193bad8333c98a0/out" "-DCMAKE_C_FLAGS= -ffunction-sections -fdata-sections -fPIC -m64" "-DCMAKE_C_COMPILER=/usr/bin/cc" "-DCMAKE_CXX_FLAGS= -ffunction-sections -fdata-sections -fPIC -m64" "-DCMAKE_CXX_COMPILER=/usr/bin/c++" "-DCMAKE_ASM_FLAGS= -ffunction-sections -fdata-sections -fPIC -m64" "-DCMAKE_ASM_COMPILER=/usr/bin/cc"
  -- The C compiler identification is GNU 8.3.0
  -- The CXX compiler identification is GNU 8.3.0
  -- Check for working C compiler: /usr/bin/cc
  -- Check for working C compiler: /usr/bin/cc -- works
  -- Detecting C compiler ABI info
  -- Detecting C compiler ABI info - done
  -- Detecting C compile features
  -- Detecting C compile features - done
  -- Check for working CXX compiler: /usr/bin/c++
  -- Check for working CXX compiler: /usr/bin/c++ -- works
  -- Detecting CXX compiler ABI info
  -- Detecting CXX compiler ABI info - done
  -- Detecting CXX compile features
  -- Detecting CXX compile features - done
  -- Configuring done
  -- Generating done
  -- Build files have been written to: /home/vscode/clib-example/target/debug/build/clib-graphviz-8193bad8333c98a0/out/build
  running: "cmake" "--build" "." "--target" "install" "--config" "Debug" "--parallel" "12"
  Scanning dependencies of target ingraphs
  Scanning dependencies of target cdt
  Scanning dependencies of target osage
  Scanning dependencies of target label
  Scanning dependencies of target fdpgen
  Scanning dependencies of target ortho
  Scanning dependencies of target patchwork
  Scanning dependencies of target circogen
  [  0%] Building C object lib/ingraphs/CMakeFiles/ingraphs.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/ingraphs/ingraphs.c.o
  Scanning dependencies of target dotgen
  Scanning dependencies of target pack_obj
  [  0%] Building C object lib/label/CMakeFiles/label.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/label/index.c.o
  [  0%] Building C object lib/osage/CMakeFiles/osage.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/osage/osageinit.c.o
  [  0%] Building C object lib/cdt/CMakeFiles/cdt.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/cdt/dtclose.c.o
  [  0%] Building C object lib/ortho/CMakeFiles/ortho.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/ortho/fPQ.c.o
  Scanning dependencies of target common
  [  0%] Building C object lib/dotgen/CMakeFiles/dotgen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/dotgen/aspect.c.o
  [  0%] Building C object lib/pack/CMakeFiles/pack_obj.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/pack/ccomps.c.o
  [  0%] Building C object lib/circogen/CMakeFiles/circogen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/circogen/block.c.o
  [  1%] Building C object lib/fdpgen/CMakeFiles/fdpgen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/fdpgen/clusteredges.c.o
  [  1%] Building C object lib/patchwork/CMakeFiles/patchwork.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/patchwork/patchwork.c.o
  Scanning dependencies of target neatogen
  [  1%] Building C object lib/common/CMakeFiles/common.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/common/args.c.o
  [  1%] Building C object lib/neatogen/CMakeFiles/neatogen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/neatogen/adjust.c.o
  [  1%] Building C object lib/cdt/CMakeFiles/cdt.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/cdt/dtdisc.c.o
  [  2%] Linking C static library libingraphs.a
  [  3%] Building C object lib/ortho/CMakeFiles/ortho.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/ortho/maze.c.o
  [  3%] Building C object lib/circogen/CMakeFiles/circogen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/circogen/blockpath.c.o
  [  3%] Building C object lib/label/CMakeFiles/label.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/label/node.c.o
  [  4%] Building C object lib/cdt/CMakeFiles/cdt.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/cdt/dtextract.c.o
  [  4%] Built target ingraphs
  [  5%] Building C object lib/patchwork/CMakeFiles/patchwork.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/patchwork/patchworkinit.c.o
  [  6%] Building C object lib/common/CMakeFiles/common.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/common/arrows.c.o
  [  6%] Building C object lib/fdpgen/CMakeFiles/fdpgen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/fdpgen/comp.c.o
  [  7%] Building C object lib/circogen/CMakeFiles/circogen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/circogen/blocktree.c.o
  [  8%] Linking C static library libosage.a
  [  8%] Building C object lib/cdt/CMakeFiles/cdt.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/cdt/dtflatten.c.o
  [  9%] Building C object lib/label/CMakeFiles/label.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/label/rectangle.c.o
  [ 10%] Building C object lib/dotgen/CMakeFiles/dotgen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/dotgen/acyclic.c.o
  [ 11%] Building C object lib/pack/CMakeFiles/pack_obj.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/pack/pack.c.o
  [ 11%] Built target osage
  [ 11%] Building C object lib/cdt/CMakeFiles/cdt.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/cdt/dthash.c.o
  [ 11%] Building C object lib/patchwork/CMakeFiles/patchwork.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/patchwork/tree_map.c.o
  [ 12%] Building C object lib/circogen/CMakeFiles/circogen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/circogen/circpos.c.o
  [ 12%] Building C object lib/fdpgen/CMakeFiles/fdpgen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/fdpgen/dbg.c.o
  [ 12%] Building C object lib/label/CMakeFiles/label.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/label/split.q.c.o
  [ 12%] Building C object lib/circogen/CMakeFiles/circogen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/circogen/circular.c.o
  [ 12%] Building C object lib/fdpgen/CMakeFiles/fdpgen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/fdpgen/fdpinit.c.o
  [ 12%] Building C object lib/ortho/CMakeFiles/ortho.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/ortho/ortho.c.o
  [ 12%] Building C object lib/dotgen/CMakeFiles/dotgen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/dotgen/class1.c.o
  [ 13%] Building C object lib/circogen/CMakeFiles/circogen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/circogen/circularinit.c.o
  [ 14%] Building C object lib/label/CMakeFiles/label.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/label/xlabels.c.o
  [ 14%] Building C object lib/neatogen/CMakeFiles/neatogen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/neatogen/bfs.c.o
  [ 14%] Building C object lib/circogen/CMakeFiles/circogen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/circogen/deglist.c.o
  [ 14%] Linking C static library libpatchwork.a
  [ 15%] Building C object lib/cdt/CMakeFiles/cdt.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/cdt/dtlist.c.o
  [ 15%] Building C object lib/fdpgen/CMakeFiles/fdpgen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/fdpgen/grid.c.o
  [ 15%] Building C object lib/common/CMakeFiles/common.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/common/colxlate.c.o
  [ 15%] Building C object lib/dotgen/CMakeFiles/dotgen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/dotgen/class2.c.o
  [ 16%] Building C object lib/circogen/CMakeFiles/circogen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/circogen/edgelist.c.o
  [ 16%] Built target patchwork
  Scanning dependencies of target pathplan
  [ 17%] Building C object lib/neatogen/CMakeFiles/neatogen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/neatogen/call_tri.c.o
  [ 17%] Building C object lib/fdpgen/CMakeFiles/fdpgen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/fdpgen/layout.c.o
  [ 18%] Building C object lib/pathplan/CMakeFiles/pathplan.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/pathplan/cvt.c.o
  Scanning dependencies of target rbtree
  [ 18%] Building C object lib/cdt/CMakeFiles/cdt.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/cdt/dtmethod.c.o
  [ 19%] Building C object lib/rbtree/CMakeFiles/rbtree.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/rbtree/misc.c.o
  [ 19%] Building C object lib/circogen/CMakeFiles/circogen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/circogen/nodelist.c.o
  Scanning dependencies of target sparse
  [ 20%] Building C object lib/sparse/CMakeFiles/sparse.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/sparse/BinaryHeap.c.o
  [ 20%] Building C object lib/rbtree/CMakeFiles/rbtree.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/rbtree/red_black_tree.c.o
  [ 21%] Linking C static library liblabel.a
  [ 21%] Building C object lib/neatogen/CMakeFiles/neatogen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/neatogen/circuit.c.o
  [ 21%] Building C object lib/pathplan/CMakeFiles/pathplan.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/pathplan/inpoly.c.o
  [ 22%] Building C object lib/cdt/CMakeFiles/cdt.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/cdt/dtopen.c.o
  [ 23%] Building C object lib/dotgen/CMakeFiles/dotgen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/dotgen/cluster.c.o
  [ 23%] Built target label
  [ 23%] Building C object lib/pathplan/CMakeFiles/pathplan.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/pathplan/route.c.o
  [ 23%] Building C object lib/cdt/CMakeFiles/cdt.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/cdt/dtrenew.c.o
  [ 23%] Building C object lib/circogen/CMakeFiles/circogen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/circogen/nodeset.c.o
  [ 23%] Building C object lib/common/CMakeFiles/common.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/common/ellipse.c.o
  Scanning dependencies of target sfdpgen
  [ 23%] Building C object lib/sfdpgen/CMakeFiles/sfdpgen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/sfdpgen/Multilevel.c.o
  [ 24%] Building C object lib/neatogen/CMakeFiles/neatogen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/neatogen/closest.c.o
  [ 24%] Building C object lib/sparse/CMakeFiles/sparse.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/sparse/clustering.c.o
  [ 24%] Building C object lib/rbtree/CMakeFiles/rbtree.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/rbtree/stack.c.o
  [ 24%] Building C object lib/cdt/CMakeFiles/cdt.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/cdt/dtrestore.c.o
  [ 25%] Linking C static library libcircogen.a
  [ 26%] Building C object lib/cdt/CMakeFiles/cdt.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/cdt/dtsize.c.o
  [ 27%] Linking C static library librbtree.a
  [ 27%] Built target pack_obj
  [ 27%] Building C object lib/ortho/CMakeFiles/ortho.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/ortho/partition.c.o
  [ 28%] Building C object lib/common/CMakeFiles/common.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/common/emit.c.o
  [ 28%] Built target circogen
  [ 28%] Building C object lib/sfdpgen/CMakeFiles/sfdpgen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/sfdpgen/post_process.c.o
  [ 29%] Building C object lib/cdt/CMakeFiles/cdt.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/cdt/dtstat.c.o
  [ 29%] Building C object lib/sfdpgen/CMakeFiles/sfdpgen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/sfdpgen/PriorityQueue.c.o
  [ 29%] Built target rbtree
  [ 29%] Building C object lib/fdpgen/CMakeFiles/fdpgen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/fdpgen/tlayout.c.o
  [ 29%] Building C object lib/dotgen/CMakeFiles/dotgen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/dotgen/compound.c.o
  [ 30%] Building C object lib/pathplan/CMakeFiles/pathplan.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/pathplan/shortest.c.o
  [ 30%] Building C object lib/neatogen/CMakeFiles/neatogen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/neatogen/compute_hierarchy.c.o
  Scanning dependencies of target twopigen
  [ 30%] Building C object lib/sparse/CMakeFiles/sparse.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/sparse/color_palette.c.o
  [ 31%] Building C object lib/twopigen/CMakeFiles/twopigen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/twopigen/circle.c.o
  [ 32%] Building C object lib/cdt/CMakeFiles/cdt.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/cdt/dtstrhash.c.o
  [ 32%] Building C object lib/twopigen/CMakeFiles/twopigen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/twopigen/twopiinit.c.o
  [ 33%] Building C object lib/neatogen/CMakeFiles/neatogen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/neatogen/conjgrad.c.o
  [ 33%] Building C object lib/cdt/CMakeFiles/cdt.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/cdt/dttree.c.o
  [ 34%] Building C object lib/sparse/CMakeFiles/sparse.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/sparse/colorutil.c.o
  [ 34%] Linking C static library libtwopigen.a
  [ 34%] Building C object lib/cdt/CMakeFiles/cdt.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/cdt/dtview.c.o
  [ 35%] Building C object lib/dotgen/CMakeFiles/dotgen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/dotgen/conc.c.o
  [ 36%] Building C object lib/ortho/CMakeFiles/ortho.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/ortho/rawgraph.c.o
  [ 36%] Building C object lib/pathplan/CMakeFiles/pathplan.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/pathplan/shortestpth.c.o
  [ 36%] Building C object lib/neatogen/CMakeFiles/neatogen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/neatogen/constrained_majorization.c.o
  [ 37%] Building C object lib/fdpgen/CMakeFiles/fdpgen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/fdpgen/xlayout.c.o
  [ 37%] Built target twopigen
  [ 37%] Building C object lib/sparse/CMakeFiles/sparse.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/sparse/DotIO.c.o
  [ 39%] Building C object lib/cdt/CMakeFiles/cdt.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/cdt/dtwalk.c.o
  [ 39%] Building C object lib/pathplan/CMakeFiles/pathplan.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/pathplan/solvers.c.o
  [ 39%] Building C object lib/sparse/CMakeFiles/sparse.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/sparse/general.c.o
  [ 39%] Building C object lib/pathplan/CMakeFiles/pathplan.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/pathplan/triang.c.o
  [ 39%] Building C object lib/ortho/CMakeFiles/ortho.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/ortho/sgraph.c.o
  [ 39%] Building C object lib/sfdpgen/CMakeFiles/sfdpgen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/sfdpgen/sfdpinit.c.o
  [ 39%] Linking C static library libcdt.a
  [ 39%] Building C object lib/pathplan/CMakeFiles/pathplan.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/pathplan/util.c.o
  [ 40%] Building C object lib/sfdpgen/CMakeFiles/sfdpgen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/sfdpgen/sparse_solve.c.o
  [ 40%] Building C object lib/dotgen/CMakeFiles/dotgen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/dotgen/decomp.c.o
  [ 40%] Linking C static library libfdpgen.a
  [ 40%] Building C object lib/sfdpgen/CMakeFiles/sfdpgen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/sfdpgen/spring_electrical.c.o
  [ 40%] Built target cdt
  [ 40%] Building C object lib/ortho/CMakeFiles/ortho.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/ortho/trapezoid.c.o
  [ 40%] Building C object lib/sfdpgen/CMakeFiles/sfdpgen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/sfdpgen/stress_model.c.o
  [ 41%] Building C object lib/pathplan/CMakeFiles/pathplan.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/pathplan/visibility.c.o
  [ 42%] Building C object lib/sfdpgen/CMakeFiles/sfdpgen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/sfdpgen/uniform_stress.c.o
  [ 42%] Built target fdpgen
  [ 42%] Building C object lib/dotgen/CMakeFiles/dotgen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/dotgen/dotinit.c.o
  [ 42%] Building C object lib/neatogen/CMakeFiles/neatogen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/neatogen/constraint.c.o
  Scanning dependencies of target vpsc
  [ 43%] Building CXX object lib/vpsc/CMakeFiles/vpsc.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/vpsc/block.cpp.o
  [ 43%] Building C object lib/common/CMakeFiles/common.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/common/geom.c.o
  [ 43%] Building CXX object lib/vpsc/CMakeFiles/vpsc.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/vpsc/blocks.cpp.o
  [ 43%] Building C object lib/common/CMakeFiles/common.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/common/globals.c.o
  [ 44%] Building C object lib/common/CMakeFiles/common.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/common/htmllex.c.o
  [ 45%] Building C object lib/sparse/CMakeFiles/sparse.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/sparse/IntStack.c.o
  [ 45%] Linking C static library libpathplan.a
  [ 45%] Building CXX object lib/vpsc/CMakeFiles/vpsc.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/vpsc/constraint.cpp.o
  [ 45%] Building C object lib/common/CMakeFiles/common.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/common/htmltable.c.o
  [ 46%] Building C object lib/common/CMakeFiles/common.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/common/input.c.o
  [ 46%] Building C object lib/sparse/CMakeFiles/sparse.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/sparse/LinkedList.c.o
  [ 46%] Built target pathplan
  Scanning dependencies of target xdot
  [ 47%] Building C object lib/dotgen/CMakeFiles/dotgen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/dotgen/dotsplines.c.o
  [ 47%] Building C object lib/xdot/CMakeFiles/xdot.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/xdot/xdot.c.o
  [ 48%] Linking C static library libortho.a
  [ 49%] Building C object lib/sparse/CMakeFiles/sparse.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/sparse/mq.c.o
  [ 49%] Built target ortho
  [ 50%] Building C object lib/neatogen/CMakeFiles/neatogen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/neatogen/delaunay.c.o
  Scanning dependencies of target cgraph
  [ 50%] Building C object lib/cgraph/CMakeFiles/cgraph.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/cgraph/agerror.c.o
  [ 50%] Building C object lib/neatogen/CMakeFiles/neatogen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/neatogen/dijkstra.c.o
  [ 51%] Building C object lib/cgraph/CMakeFiles/cgraph.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/cgraph/agxbuf.c.o
  [ 51%] Building C object lib/sparse/CMakeFiles/sparse.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/sparse/QuadTree.c.o
  [ 51%] Building C object lib/common/CMakeFiles/common.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/common/intset.c.o
  [ 51%] Building C object lib/cgraph/CMakeFiles/cgraph.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/cgraph/apply.c.o
  [ 52%] Linking C static library libxdot.a
  [ 52%] Building C object lib/common/CMakeFiles/common.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/common/labels.c.o
  [ 53%] Building C object lib/cgraph/CMakeFiles/cgraph.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/cgraph/attr.c.o
  [ 53%] Built target xdot
  [ 53%] Building C object lib/neatogen/CMakeFiles/neatogen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/neatogen/edges.c.o
  Scanning dependencies of target pack
  [ 53%] Linking C static library libpack.a
  [ 54%] Building CXX object lib/vpsc/CMakeFiles/vpsc.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/vpsc/csolve_VPSC.cpp.o
  [ 54%] Built target pack
  [ 54%] Building CXX object lib/vpsc/CMakeFiles/vpsc.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/vpsc/generate-constraints.cpp.o
  [ 55%] Building C object lib/neatogen/CMakeFiles/neatogen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/neatogen/embed_graph.c.o
  [ 56%] Building C object lib/common/CMakeFiles/common.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/common/ns.c.o
  [ 56%] Building C object lib/common/CMakeFiles/common.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/common/memory.c.o
  [ 56%] Building C object lib/cgraph/CMakeFiles/cgraph.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/cgraph/edge.c.o
  [ 57%] Building CXX object lib/vpsc/CMakeFiles/vpsc.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/vpsc/pairingheap/PairingHeap.cpp.o
  [ 57%] Building C object lib/sparse/CMakeFiles/sparse.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/sparse/SparseMatrix.c.o
  [ 58%] Building C object lib/common/CMakeFiles/common.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/common/output.c.o
  [ 58%] Building C object lib/neatogen/CMakeFiles/neatogen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/neatogen/geometry.c.o
  [ 58%] Linking C static library libsfdpgen.a
  [ 58%] Building C object lib/common/CMakeFiles/common.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/common/pointset.c.o
  [ 59%] Building C object lib/neatogen/CMakeFiles/neatogen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/neatogen/heap.c.o
  [ 59%] Built target sfdpgen
  [ 59%] Building C object lib/neatogen/CMakeFiles/neatogen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/neatogen/hedges.c.o
  [ 59%] Building C object lib/cgraph/CMakeFiles/cgraph.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/cgraph/flatten.c.o
  [ 59%] Building C object lib/common/CMakeFiles/common.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/common/postproc.c.o
  [ 59%] Building CXX object lib/vpsc/CMakeFiles/vpsc.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/vpsc/solve_VPSC.cpp.o
  [ 59%] Building C object lib/neatogen/CMakeFiles/neatogen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/neatogen/info.c.o
  [ 60%] Building C object lib/cgraph/CMakeFiles/cgraph.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/cgraph/graph.c.o
  [ 61%] Building C object lib/neatogen/CMakeFiles/neatogen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/neatogen/kkutils.c.o
  [ 61%] Building C object lib/cgraph/CMakeFiles/cgraph.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/cgraph/id.c.o
  [ 61%] Building C object lib/neatogen/CMakeFiles/neatogen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/neatogen/legal.c.o
  [ 62%] Building C object lib/cgraph/CMakeFiles/cgraph.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/cgraph/imap.c.o
  [ 62%] Building CXX object lib/vpsc/CMakeFiles/vpsc.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/vpsc/variable.cpp.o
  [ 62%] Building C object lib/cgraph/CMakeFiles/cgraph.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/cgraph/io.c.o
  [ 63%] Building C object lib/common/CMakeFiles/common.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/common/psusershape.c.o
  [ 64%] Building C object lib/neatogen/CMakeFiles/neatogen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/neatogen/lu.c.o
  [ 65%] Building C object lib/sparse/CMakeFiles/sparse.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/sparse/vector.c.o
  [ 65%] Building C object lib/cgraph/CMakeFiles/cgraph.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/cgraph/mem.c.o
  [ 65%] Building C object lib/common/CMakeFiles/common.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/common/routespl.c.o
  [ 65%] Building C object lib/neatogen/CMakeFiles/neatogen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/neatogen/matinv.c.o
  [ 66%] Building C object lib/cgraph/CMakeFiles/cgraph.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/cgraph/node.c.o
  [ 66%] Building C object lib/cgraph/CMakeFiles/cgraph.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/cgraph/obj.c.o
  [ 66%] Building C object lib/neatogen/CMakeFiles/neatogen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/neatogen/matrix_ops.c.o
  [ 67%] Building C object lib/neatogen/CMakeFiles/neatogen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/neatogen/memory.c.o
  [ 67%] Building C object lib/common/CMakeFiles/common.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/common/shapes.c.o
  [ 67%] Building C object lib/neatogen/CMakeFiles/neatogen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/neatogen/multispline.c.o
  [ 67%] Building C object lib/dotgen/CMakeFiles/dotgen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/dotgen/fastgr.c.o
  [ 67%] Building C object lib/cgraph/CMakeFiles/cgraph.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/cgraph/pend.c.o
  [ 67%] Building C object lib/neatogen/CMakeFiles/neatogen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/neatogen/neatoinit.c.o
  [ 68%] Building C object lib/cgraph/CMakeFiles/cgraph.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/cgraph/rec.c.o
  [ 68%] Building C object lib/cgraph/CMakeFiles/cgraph.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/cgraph/refstr.c.o
  [ 69%] Building C object lib/cgraph/CMakeFiles/cgraph.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/cgraph/subg.c.o
  [ 70%] Building C object lib/dotgen/CMakeFiles/dotgen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/dotgen/flat.c.o
  [ 71%] Building C object lib/neatogen/CMakeFiles/neatogen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/neatogen/neatosplines.c.o
  [ 71%] Building C object lib/cgraph/CMakeFiles/cgraph.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/cgraph/utils.c.o
  [ 71%] Building C object lib/cgraph/CMakeFiles/cgraph.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/cgraph/write.c.o
  [ 72%] Building C object lib/common/CMakeFiles/common.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/common/splines.c.o
  [ 72%] Building C object lib/neatogen/CMakeFiles/neatogen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/neatogen/opt_arrangement.c.o
  [ 73%] Building C object lib/cgraph/CMakeFiles/cgraph.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/build/lib/cgraph/grammar.c.o
  [ 73%] Linking C static library libsparse.a
  [ 73%] Building C object lib/dotgen/CMakeFiles/dotgen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/dotgen/mincross.c.o
  [ 73%] Building C object lib/dotgen/CMakeFiles/dotgen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/dotgen/position.c.o
  [ 73%] Built target sparse
  [ 74%] Building C object lib/neatogen/CMakeFiles/neatogen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/neatogen/overlap.c.o
  [ 74%] Building C object lib/neatogen/CMakeFiles/neatogen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/neatogen/pca.c.o
  [ 74%] Building C object lib/cgraph/CMakeFiles/cgraph.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/build/lib/cgraph/scan.c.o
  [ 74%] Building C object lib/neatogen/CMakeFiles/neatogen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/neatogen/poly.c.o
  [ 75%] Building C object lib/neatogen/CMakeFiles/neatogen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/neatogen/printvis.c.o
  [ 75%] Building C object lib/neatogen/CMakeFiles/neatogen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/neatogen/quad_prog_solve.c.o
  [ 76%] Building C object lib/neatogen/CMakeFiles/neatogen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/neatogen/site.c.o
  [ 76%] Building C object lib/neatogen/CMakeFiles/neatogen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/neatogen/smart_ini_x.c.o
  [ 76%] Building C object lib/neatogen/CMakeFiles/neatogen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/neatogen/solve.c.o
  [ 76%] Building C object lib/common/CMakeFiles/common.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/common/taper.c.o
  [ 77%] Building C object lib/neatogen/CMakeFiles/neatogen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/neatogen/stuff.c.o
  [ 77%] Building C object lib/neatogen/CMakeFiles/neatogen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/neatogen/stress.c.o
  [ 77%] Building C object lib/neatogen/CMakeFiles/neatogen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/neatogen/voronoi.c.o
  [ 78%] Building C object lib/neatogen/CMakeFiles/neatogen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/neatogen/sgd.c.o
  [ 79%] Building C object lib/common/CMakeFiles/common.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/common/textspan.c.o
  [ 79%] Building C object lib/neatogen/CMakeFiles/neatogen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/neatogen/randomkit.c.o
  [ 79%] Building C object lib/common/CMakeFiles/common.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/common/textspan_lut.c.o
  [ 80%] Building C object lib/neatogen/CMakeFiles/neatogen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/neatogen/constrained_majorization_ipsep.c.o
  [ 82%] Linking C static library libcgraph.a
  [ 82%] Building C object lib/dotgen/CMakeFiles/dotgen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/dotgen/rank.c.o
  [ 83%] Linking CXX static library libvpsc.a
  [ 83%] Building C object lib/neatogen/CMakeFiles/neatogen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/neatogen/mosek_quad_solve.c.o
  [ 83%] Building C object lib/common/CMakeFiles/common.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/common/timing.c.o
  [ 83%] Building C object lib/neatogen/CMakeFiles/neatogen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/neatogen/quad_prog_vpsc.c.o
  [ 84%] Building C object lib/common/CMakeFiles/common.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/common/utils.c.o
  [ 84%] Building C object lib/common/CMakeFiles/common.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/common/xml.c.o
  [ 84%] Built target cgraph
  [ 84%] Built target vpsc
  [ 85%] Building C object lib/common/CMakeFiles/common.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/build/lib/common/htmlparse.c.o
  Scanning dependencies of target gvc
  [ 85%] Building C object lib/dotgen/CMakeFiles/dotgen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/dotgen/sameport.c.o
  [ 85%] Building C object lib/gvc/CMakeFiles/gvc.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/gvc/gvc.c.o
  [ 85%] Building C object lib/gvc/CMakeFiles/gvc.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/gvc/gvconfig.c.o
  [ 86%] Building C object lib/gvc/CMakeFiles/gvc.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/gvc/gvcontext.c.o
  [ 86%] Building C object lib/gvc/CMakeFiles/gvc.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/gvc/gvdevice.c.o
  [ 87%] Building C object lib/gvc/CMakeFiles/gvc.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/gvc/gvevent.c.o
  [ 87%] Building C object lib/gvc/CMakeFiles/gvc.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/gvc/gvjobs.c.o
  [ 87%] Building C object lib/gvc/CMakeFiles/gvc.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/gvc/gvlayout.c.o
  [ 88%] Building C object lib/gvc/CMakeFiles/gvc.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/gvc/gvloadimage.c.o
  [ 88%] Building C object lib/gvc/CMakeFiles/gvc.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/gvc/gvplugin.c.o
  [ 89%] Building C object lib/gvc/CMakeFiles/gvc.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/gvc/gvrender.c.o
  [ 89%] Building C object lib/gvc/CMakeFiles/gvc.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/gvc/gvtextlayout.c.o
  [ 89%] Building C object lib/gvc/CMakeFiles/gvc.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/gvc/gvtool_tred.c.o
  [ 90%] Building C object lib/gvc/CMakeFiles/gvc.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/gvc/gvusershape.c.o
  [ 91%] Linking C static library libneatogen.a
  [ 91%] Linking C static library libdotgen.a
  [ 91%] Built target dotgen
  [ 91%] Built target neatogen
  [ 91%] Linking C static library libgvc.a
  [ 91%] Built target gvc
  Scanning dependencies of target gvplugin_dot_layout
  Scanning dependencies of target gvplugin_neato_layout
  Scanning dependencies of target gvplugin_core
  [ 92%] Building C object plugin/neato_layout/CMakeFiles/gvplugin_neato_layout.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/plugin/neato_layout/gvlayout_neato_layout.c.o
  [ 92%] Building C object plugin/dot_layout/CMakeFiles/gvplugin_dot_layout.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/plugin/dot_layout/gvplugin_dot_layout.c.o
  [ 92%] Building C object plugin/dot_layout/CMakeFiles/gvplugin_dot_layout.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/plugin/dot_layout/gvlayout_dot_layout.c.o
  [ 92%] Building C object plugin/neato_layout/CMakeFiles/gvplugin_neato_layout.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/plugin/neato_layout/gvplugin_neato_layout.c.o
  [ 93%] Building C object plugin/core/CMakeFiles/gvplugin_core.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/plugin/core/gvplugin_core.c.o
  [ 94%] Building C object plugin/core/CMakeFiles/gvplugin_core.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/plugin/core/gvrender_core_json.c.o
  [ 94%] Building C object plugin/core/CMakeFiles/gvplugin_core.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/plugin/core/gvloadimage_core.c.o
  [ 94%] Building C object plugin/core/CMakeFiles/gvplugin_core.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/plugin/core/gvrender_core_dot.c.o
  [ 95%] Building C object plugin/core/CMakeFiles/gvplugin_core.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/plugin/core/gvrender_core_mp.c.o
  [ 95%] Building C object plugin/core/CMakeFiles/gvplugin_core.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/plugin/core/gvrender_core_fig.c.o
  [ 95%] Building C object plugin/core/CMakeFiles/gvplugin_core.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/plugin/core/gvrender_core_map.c.o
  [ 95%] Linking C static library libcommon.a
  [ 95%] Building C object plugin/core/CMakeFiles/gvplugin_core.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/plugin/core/gvrender_core_pic.c.o
  [ 95%] Building C object plugin/core/CMakeFiles/gvplugin_core.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/plugin/core/gvrender_core_ps.c.o
  [ 96%] Building C object plugin/core/CMakeFiles/gvplugin_core.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/plugin/core/gvrender_core_pov.c.o
  [ 97%] Linking C static library libgvplugin_dot_layout.a
  [ 98%] Linking CXX static library libgvplugin_neato_layout.a
  [ 98%] Built target common
  [ 98%] Building C object plugin/core/CMakeFiles/gvplugin_core.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/plugin/core/gvrender_core_svg.c.o
  [ 98%] Built target gvplugin_dot_layout
  [ 98%] Built target gvplugin_neato_layout
  [ 99%] Building C object plugin/core/CMakeFiles/gvplugin_core.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/plugin/core/gvrender_core_tk.c.o
  [ 99%] Building C object plugin/core/CMakeFiles/gvplugin_core.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/plugin/core/gvrender_core_vml.c.o
  [ 99%] Linking C static library libgvplugin_core.a
  [ 99%] Built target gvplugin_core
  Scanning dependencies of target graphvizlib
  [ 99%] Building CXX object graphvizlib/CMakeFiles/graphvizlib.dir/main.cpp.o
  [100%] Linking CXX executable graphvizlib

  --- stderr
  CMake Warning:
    Manually-specified variables were not used by the project:

      CMAKE_ASM_COMPILER
      CMAKE_ASM_FLAGS


  make: warning: -j12 forced in submake: resetting jobserver mode.
  /usr/bin/ld: /usr/lib/gcc/x86_64-linux-gnu/8/../../../x86_64-linux-gnu/Scrt1.o: in function `_start':
  (.text+0x20): undefined reference to `main'
  /usr/bin/ld: ../lib/dotgen/libdotgen.a(dotinit.c.o): in function `dot_layout':
  dotinit.c:(.text.dot_layout+0x38b): undefined reference to `dotneato_postprocess'
  /usr/bin/ld: ../lib/dotgen/libdotgen.a(dotsplines.c.o): in function `_dot_splines':
  dotsplines.c:(.text._dot_splines+0x139): undefined reference to `orthoEdges'
  /usr/bin/ld: dotsplines.c:(.text._dot_splines+0x14e): undefined reference to `orthoEdges'
  /usr/bin/ld: dotsplines.c:(.text._dot_splines+0x1c1): undefined reference to `routesplinesterm'
  /usr/bin/ld: dotsplines.c:(.text._dot_splines+0x308): undefined reference to `routesplinesinit'
  /usr/bin/ld: dotsplines.c:(.text._dot_splines+0x734): undefined reference to `makeStraightEdges'
  /usr/bin/ld: dotsplines.c:(.text._dot_splines+0x167c): undefined reference to `routesplines'
  /usr/bin/ld: dotsplines.c:(.text._dot_splines+0x1845): undefined reference to `routepolylines'
  /usr/bin/ld: dotsplines.c:(.text._dot_splines+0x1ba9): undefined reference to `routesplines'
  /usr/bin/ld: dotsplines.c:(.text._dot_splines+0x1bb3): undefined reference to `routepolylines'
  /usr/bin/ld: ../lib/dotgen/libdotgen.a(dotsplines.c.o): in function `make_flat_edge':
  dotsplines.c:(.text.make_flat_edge+0x6f3): undefined reference to `simpleSplineRoute'
  /usr/bin/ld: dotsplines.c:(.text.make_flat_edge+0x9fd): undefined reference to `simpleSplineRoute'
  /usr/bin/ld: dotsplines.c:(.text.make_flat_edge+0x159b): undefined reference to `dotneato_postprocess'
  /usr/bin/ld: dotsplines.c:(.text.make_flat_edge+0x1f9d): undefined reference to `routesplines'
  /usr/bin/ld: dotsplines.c:(.text.make_flat_edge+0x1fa4): undefined reference to `routepolylines'
  /usr/bin/ld: dotsplines.c:(.text.make_flat_edge+0x2344): undefined reference to `routesplines'
  /usr/bin/ld: dotsplines.c:(.text.make_flat_edge+0x234b): undefined reference to `routepolylines'
  /usr/bin/ld: dotsplines.c:(.text.make_flat_edge+0x269a): undefined reference to `routesplines'
  /usr/bin/ld: dotsplines.c:(.text.make_flat_edge+0x26a1): undefined reference to `routepolylines'
  /usr/bin/ld: ../lib/dotgen/libdotgen.a(rank.c.o): in function `make_new_cluster':
  rank.c:(.text.make_new_cluster+0x64): undefined reference to `do_graph_label'
  /usr/bin/ld: ../lib/gvc/libgvc.a(gvevent.c.o): in function `gvevent_read':
  gvevent.c:(.text.gvevent_read+0x90): undefined reference to `graph_cleanup'
  /usr/bin/ld: ../lib/gvc/libgvc.a(gvlayout.c.o): in function `gvLayoutJobs':
  gvlayout.c:(.text.gvLayoutJobs+0xd9): undefined reference to `graph_init'
  /usr/bin/ld: ../lib/gvc/libgvc.a(gvlayout.c.o): in function `gvFreeLayout':
  gvlayout.c:(.text.gvFreeLayout+0x52): undefined reference to `graph_cleanup'
  /usr/bin/ld: ../lib/pack/libpack.a(pack.c.o): in function `pack_graph':
  pack.c:(.text.pack_graph+0x5c): undefined reference to `dotneato_postprocess'
  /usr/bin/ld: ../lib/circogen/libcircogen.a(circularinit.c.o): in function `circo_layout':
  circularinit.c:(.text.circo_layout+0x42): undefined reference to `dotneato_postprocess'
  /usr/bin/ld: ../lib/fdpgen/libfdpgen.a(layout.c.o): in function `layout':
  layout.c:(.text.layout+0x16a): undefined reference to `do_graph_label'
  /usr/bin/ld: ../lib/fdpgen/libfdpgen.a(layout.c.o): in function `fdp_layout':
  layout.c:(.text.fdp_layout+0x2bd): undefined reference to `gv_postprocess'
  /usr/bin/ld: ../lib/fdpgen/libfdpgen.a(clusteredges.c.o): in function `compoundEdges':
  clusteredges.c:(.text.compoundEdges+0x2a7): undefined reference to `Pobsopen'
  /usr/bin/ld: ../lib/fdpgen/libfdpgen.a(grid.c.o): in function `walkGrid':
  grid.c:(.text.walkGrid+0x7): undefined reference to `dtwalk'
  /usr/bin/ld: ../lib/neatogen/libneatogen.a(legal.c.o): in function `Plegal_arrangement':
  legal.c:(.text.Plegal_arrangement+0x651): undefined reference to `in_poly'
  /usr/bin/ld: ../lib/neatogen/libneatogen.a(neatoinit.c.o): in function `dfs':
  neatoinit.c:(.text.dfs+0x129): undefined reference to `do_graph_label'
  /usr/bin/ld: ../lib/neatogen/libneatogen.a(neatoinit.c.o): in function `neato_layout':
  neatoinit.c:(.text.neato_layout+0x320): undefined reference to `gv_postprocess'
  /usr/bin/ld: ../lib/neatogen/libneatogen.a(neatosplines.c.o): in function `getPath':
  neatosplines.c:(.text.getPath+0x80): undefined reference to `Pobspath'
  /usr/bin/ld: ../lib/neatogen/libneatogen.a(neatosplines.c.o): in function `makeSpline':
  neatosplines.c:(.text.makeSpline+0xbd): undefined reference to `in_poly'
  /usr/bin/ld: neatosplines.c:(.text.makeSpline+0xef): undefined reference to `in_poly'
  /usr/bin/ld: neatosplines.c:(.text.makeSpline+0x1f2): undefined reference to `Proutespline'
  /usr/bin/ld: ../lib/neatogen/libneatogen.a(neatosplines.c.o): in function `_spline_edges':
  neatosplines.c:(.text._spline_edges+0xcd): undefined reference to `Pobsopen'
  /usr/bin/ld: neatosplines.c:(.text._spline_edges+0x1ff): undefined reference to `orthoEdges'
  /usr/bin/ld: neatosplines.c:(.text._spline_edges+0x4aa): undefined reference to `makeStraightEdge'
  /usr/bin/ld: neatosplines.c:(.text._spline_edges+0x4de): undefined reference to `Pobsclose'
  /usr/bin/ld: ../lib/neatogen/libneatogen.a(stuff.c.o): in function `scan_graph_mode':
  stuff.c:(.text.scan_graph_mode+0x173): undefined reference to `getdouble'
  /usr/bin/ld: stuff.c:(.text.scan_graph_mode+0x26c): undefined reference to `getdouble'
  /usr/bin/ld: stuff.c:(.text.scan_graph_mode+0x321): undefined reference to `getdouble'
  /usr/bin/ld: ../lib/osage/libosage.a(osageinit.c.o): in function `mkClusters':
  osageinit.c:(.text.mkClusters+0x87): undefined reference to `do_graph_label'
  /usr/bin/ld: ../lib/osage/libosage.a(osageinit.c.o): in function `osage_layout':
  osageinit.c:(.text.osage_layout+0x16c): undefined reference to `dotneato_postprocess'
  /usr/bin/ld: ../lib/patchwork/libpatchwork.a(patchworkinit.c.o): in function `patchwork_layout':
  patchworkinit.c:(.text.patchwork_layout+0x186): undefined reference to `dotneato_postprocess'
  /usr/bin/ld: ../lib/twopigen/libtwopigen.a(twopiinit.c.o): in function `twopi_layout':
  twopiinit.c:(.text.twopi_layout+0x308): undefined reference to `dotneato_postprocess'
  /usr/bin/ld: ../lib/sfdpgen/libsfdpgen.a(sfdpinit.c.o): in function `sfdp_layout':
  sfdpinit.c:(.text.sfdp_layout+0x68b): undefined reference to `dotneato_postprocess'
  /usr/bin/ld: ../lib/sfdpgen/libsfdpgen.a(spring_electrical.c.o): in function `beautify_leaves':
  spring_electrical.c:(.text.beautify_leaves+0x15c): undefined reference to `distance'
  /usr/bin/ld: ../lib/sfdpgen/libsfdpgen.a(spring_electrical.c.o): in function `spring_maxent_embedding':
  spring_electrical.c:(.text.spring_maxent_embedding+0x34a): undefined reference to `drand'
  /usr/bin/ld: spring_electrical.c:(.text.spring_maxent_embedding+0x398): undefined reference to `distance_cropped'
  /usr/bin/ld: spring_electrical.c:(.text.spring_maxent_embedding+0x537): undefined reference to `QuadTree_new_from_point_list'
  /usr/bin/ld: spring_electrical.c:(.text.spring_maxent_embedding+0x5d1): undefined reference to `distance_cropped'
  /usr/bin/ld: spring_electrical.c:(.text.spring_maxent_embedding+0x841): undefined reference to `QuadTree_get_supernodes'
  /usr/bin/ld: spring_electrical.c:(.text.spring_maxent_embedding+0x9ad): undefined reference to `distance_cropped'
  /usr/bin/ld: spring_electrical.c:(.text.spring_maxent_embedding+0xa92): undefined reference to `distance_cropped'
  /usr/bin/ld: spring_electrical.c:(.text.spring_maxent_embedding+0xbfb): undefined reference to `QuadTree_delete'
  /usr/bin/ld: ../lib/sfdpgen/libsfdpgen.a(spring_electrical.c.o): in function `spring_electrical_embedding_fast':
  spring_electrical.c:(.text.spring_electrical_embedding_fast+0x150): undefined reference to `drand'
  /usr/bin/ld: spring_electrical.c:(.text.spring_electrical_embedding_fast+0x279): undefined reference to `QuadTree_new_from_point_list'
  /usr/bin/ld: spring_electrical.c:(.text.spring_electrical_embedding_fast+0x2ad): undefined reference to `QuadTree_get_repulsive_force'
  /usr/bin/ld: spring_electrical.c:(.text.spring_electrical_embedding_fast+0x311): undefined reference to `distance'
  /usr/bin/ld: spring_electrical.c:(.text.spring_electrical_embedding_fast+0x482): undefined reference to `QuadTree_delete'
  /usr/bin/ld: ../lib/sfdpgen/libsfdpgen.a(spring_electrical.c.o): in function `spring_electrical_embedding':
  spring_electrical.c:(.text.spring_electrical_embedding+0x237): undefined reference to `drand'
  /usr/bin/ld: spring_electrical.c:(.text.spring_electrical_embedding+0x398): undefined reference to `QuadTree_new_from_point_list'
  /usr/bin/ld: spring_electrical.c:(.text.spring_electrical_embedding+0x44d): undefined reference to `distance'
  /usr/bin/ld: spring_electrical.c:(.text.spring_electrical_embedding+0x4f7): undefined reference to `QuadTree_get_supernodes'
  /usr/bin/ld: spring_electrical.c:(.text.spring_electrical_embedding+0x683): undefined reference to `distance_cropped'
  /usr/bin/ld: spring_electrical.c:(.text.spring_electrical_embedding+0x755): undefined reference to `distance_cropped'
  /usr/bin/ld: spring_electrical.c:(.text.spring_electrical_embedding+0x8a5): undefined reference to `QuadTree_delete'
  /usr/bin/ld: ../lib/sfdpgen/libsfdpgen.a(spring_electrical.c.o): in function `spring_electrical_spring_embedding':
  spring_electrical.c:(.text.spring_electrical_spring_embedding+0x22b): undefined reference to `drand'
  /usr/bin/ld: spring_electrical.c:(.text.spring_electrical_spring_embedding+0x36d): undefined reference to `QuadTree_new_from_point_list'
  /usr/bin/ld: spring_electrical.c:(.text.spring_electrical_spring_embedding+0x40b): undefined reference to `distance'
  /usr/bin/ld: spring_electrical.c:(.text.spring_electrical_spring_embedding+0x494): undefined reference to `distance_cropped'
  /usr/bin/ld: spring_electrical.c:(.text.spring_electrical_spring_embedding+0x57c): undefined reference to `QuadTree_get_supernodes'
  /usr/bin/ld: spring_electrical.c:(.text.spring_electrical_spring_embedding+0x6d3): undefined reference to `distance_cropped'
  /usr/bin/ld: spring_electrical.c:(.text.spring_electrical_spring_embedding+0x799): undefined reference to `distance_cropped'
  /usr/bin/ld: spring_electrical.c:(.text.spring_electrical_spring_embedding+0x8e3): undefined reference to `QuadTree_delete'
  /usr/bin/ld: ../lib/sfdpgen/libsfdpgen.a(spring_electrical.c.o): in function `multilevel_spring_electrical_embedding':
  spring_electrical.c:(.text.multilevel_spring_electrical_embedding+0x5dd): undefined reference to `remove_overlap'
  /usr/bin/ld: spring_electrical.c:(.text.multilevel_spring_electrical_embedding+0x8da): undefined reference to `drand'
  /usr/bin/ld: spring_electrical.c:(.text.multilevel_spring_electrical_embedding+0xa97): undefined reference to `distance_cropped'
  /usr/bin/ld: spring_electrical.c:(.text.multilevel_spring_electrical_embedding+0xba5): undefined reference to `distance_cropped'
  /usr/bin/ld: spring_electrical.c:(.text.multilevel_spring_electrical_embedding+0xd18): undefined reference to `distance'
  /usr/bin/ld: spring_electrical.c:(.text.multilevel_spring_electrical_embedding+0x1287): undefined reference to `drand'
  /usr/bin/ld: spring_electrical.c:(.text.multilevel_spring_electrical_embedding+0x1512): undefined reference to `remove_overlap'
  /usr/bin/ld: ../lib/sfdpgen/libsfdpgen.a(uniform_stress.c.o): in function `uniform_stress':
  uniform_stress.c:(.text.uniform_stress+0x36): undefined reference to `drand'
  /usr/bin/ld: uniform_stress.c:(.text.uniform_stress+0xc8): undefined reference to `drand'
  /usr/bin/ld: uniform_stress.c:(.text.uniform_stress+0x21f): undefined reference to `scale_to_box'
  /usr/bin/ld: ../lib/sfdpgen/libsfdpgen.a(Multilevel.c.o): in function `Multilevel_coarsen_internal':
  Multilevel.c:(.text.Multilevel_coarsen_internal+0x4c1): undefined reference to `random_permutation'
  /usr/bin/ld: Multilevel.c:(.text.Multilevel_coarsen_internal+0x909): undefined reference to `random_permutation'
  /usr/bin/ld: Multilevel.c:(.text.Multilevel_coarsen_internal+0xab6): undefined reference to `random_permutation'
  /usr/bin/ld: Multilevel.c:(.text.Multilevel_coarsen_internal+0xf46): undefined reference to `random_permutation'
  /usr/bin/ld: Multilevel.c:(.text.Multilevel_coarsen_internal+0x109c): undefined reference to `random_permutation'
  /usr/bin/ld: ../lib/sfdpgen/libsfdpgen.a(Multilevel.c.o):Multilevel.c:(.text.Multilevel_coarsen_internal+0x127b): more undefined references to `random_permutation' follow
  /usr/bin/ld: ../lib/sfdpgen/libsfdpgen.a(post_process.c.o): in function `ideal_distance_matrix':
  post_process.c:(.text.ideal_distance_matrix+0x163): undefined reference to `distance'
  /usr/bin/ld: ../lib/sfdpgen/libsfdpgen.a(post_process.c.o): in function `get_stress':
  post_process.c:(.text.get_stress+0x88): undefined reference to `distance'
  /usr/bin/ld: post_process.c:(.text.get_stress+0xbb): undefined reference to `distance'
  /usr/bin/ld: post_process.c:(.text.get_stress+0xcf): undefined reference to `distance'
  /usr/bin/ld: post_process.c:(.text.get_stress+0xf1): undefined reference to `distance'
  /usr/bin/ld: ../lib/sfdpgen/libsfdpgen.a(post_process.c.o): in function `StressMajorizationSmoother_smooth':
  post_process.c:(.text.StressMajorizationSmoother_smooth+0x274): undefined reference to `distance_cropped'
  /usr/bin/ld: post_process.c:(.text.StressMajorizationSmoother_smooth+0x47f): undefined reference to `distance_cropped'
  /usr/bin/ld: post_process.c:(.text.StressMajorizationSmoother_smooth+0x6b2): undefined reference to `distance'
  /usr/bin/ld: post_process.c:(.text.StressMajorizationSmoother_smooth+0x717): undefined reference to `drand'
  /usr/bin/ld: post_process.c:(.text.StressMajorizationSmoother_smooth+0x775): undefined reference to `distance'
  /usr/bin/ld: post_process.c:(.text.StressMajorizationSmoother_smooth+0x916): undefined reference to `distance_cropped'
  /usr/bin/ld: post_process.c:(.text.StressMajorizationSmoother_smooth+0xac9): undefined reference to `vector_product'
  /usr/bin/ld: ../lib/sfdpgen/libsfdpgen.a(post_process.c.o): in function `StressMajorizationSmoother2_new':
  post_process.c:(.text.StressMajorizationSmoother2_new+0x13b): undefined reference to `distance'
  /usr/bin/ld: post_process.c:(.text.StressMajorizationSmoother2_new+0x3a0): undefined reference to `distance_cropped'
  /usr/bin/ld: post_process.c:(.text.StressMajorizationSmoother2_new+0x459): undefined reference to `distance'
  /usr/bin/ld: post_process.c:(.text.StressMajorizationSmoother2_new+0x5b2): undefined reference to `distance_cropped'
  /usr/bin/ld: post_process.c:(.text.StressMajorizationSmoother2_new+0x676): undefined reference to `distance'
  /usr/bin/ld: ../lib/sfdpgen/libsfdpgen.a(post_process.c.o): in function `SparseStressMajorizationSmoother_new':
  post_process.c:(.text.SparseStressMajorizationSmoother_new+0xdf): undefined reference to `drand'
  /usr/bin/ld: post_process.c:(.text.SparseStressMajorizationSmoother_new+0x314): undefined reference to `distance'
  /usr/bin/ld: ../lib/sfdpgen/libsfdpgen.a(post_process.c.o): in function `TriangleSmoother_new':
  post_process.c:(.text.TriangleSmoother_new+0x92): undefined reference to `distance'
  /usr/bin/ld: post_process.c:(.text.TriangleSmoother_new+0x169): undefined reference to `call_tri'
  /usr/bin/ld: post_process.c:(.text.TriangleSmoother_new+0x170): undefined reference to `call_tri2'
  /usr/bin/ld: post_process.c:(.text.TriangleSmoother_new+0x262): undefined reference to `distance_cropped'
  /usr/bin/ld: post_process.c:(.text.TriangleSmoother_new+0x2d3): undefined reference to `distance'
  /usr/bin/ld: ../lib/sfdpgen/libsfdpgen.a(post_process.c.o): in function `SpringSmoother_new':
  post_process.c:(.text.SpringSmoother_new+0xc9): undefined reference to `distance'
  /usr/bin/ld: ../lib/sfdpgen/libsfdpgen.a(PriorityQueue.c.o): in function `PriorityQueue_delete':
  PriorityQueue.c:(.text.PriorityQueue_delete+0x2c): undefined reference to `DoubleLinkedList_delete'
  /usr/bin/ld: ../lib/sfdpgen/libsfdpgen.a(PriorityQueue.c.o): in function `PriorityQueue_push':
  PriorityQueue.c:(.text.PriorityQueue_push+0x70): undefined reference to `DoubleLinkedList_prepend'
  /usr/bin/ld: PriorityQueue.c:(.text.PriorityQueue_push+0x7d): undefined reference to `DoubleLinkedList_new'
  /usr/bin/ld: PriorityQueue.c:(.text.PriorityQueue_push+0xb9): undefined reference to `DoubleLinkedList_delete_element'
  /usr/bin/ld: ../lib/sfdpgen/libsfdpgen.a(PriorityQueue.c.o): in function `PriorityQueue_pop':
  PriorityQueue.c:(.text.PriorityQueue_pop+0x3e): undefined reference to `DoubleLinkedList_get_data'
  /usr/bin/ld: PriorityQueue.c:(.text.PriorityQueue_pop+0x5a): undefined reference to `DoubleLinkedList_delete_element'
  /usr/bin/ld: ../lib/sfdpgen/libsfdpgen.a(PriorityQueue.c.o): in function `PriorityQueue_remove':
  PriorityQueue.c:(.text.PriorityQueue_remove+0x46): undefined reference to `DoubleLinkedList_delete_element'
  /usr/bin/ld: ../lib/sfdpgen/libsfdpgen.a(sparse_solve.c.o): in function `cg':
  sparse_solve.c:(.text.cg+0x15d): undefined reference to `vector_subtract_to'
  /usr/bin/ld: sparse_solve.c:(.text.cg+0x16d): undefined reference to `vector_product'
  /usr/bin/ld: sparse_solve.c:(.text.cg+0x234): undefined reference to `vector_product'
  /usr/bin/ld: sparse_solve.c:(.text.cg+0x260): undefined reference to `vector_saxpy'
  /usr/bin/ld: sparse_solve.c:(.text.cg+0x298): undefined reference to `vector_product'
  /usr/bin/ld: sparse_solve.c:(.text.cg+0x2be): undefined reference to `vector_saxpy2'
  /usr/bin/ld: sparse_solve.c:(.text.cg+0x2e4): undefined reference to `vector_saxpy2'
  /usr/bin/ld: sparse_solve.c:(.text.cg+0x2f4): undefined reference to `vector_product'
  collect2: error: ld returned 1 exit status
  make[2]: *** [graphvizlib/CMakeFiles/graphvizlib.dir/build.make:108: graphvizlib/graphvizlib] Error 1
  make[1]: *** [CMakeFiles/Makefile2:1608: graphvizlib/CMakeFiles/graphvizlib.dir/all] Error 2
  make: *** [Makefile:130: all] Error 2
  thread 'main' panicked at '
  command did not execute successfully, got: exit status: 2

  build script failed, must exit now', /usr/local/cargo/registry/src/github.com-1ecc6299db9ec823/cmake-0.1.48/src/lib.rs:975:5
  note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```