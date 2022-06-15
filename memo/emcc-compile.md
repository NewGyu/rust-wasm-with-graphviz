The case when `.define("CMAKE_TOOLCHAIN_FILE", "../emsdk/upstream/emscripten/cmake/Modules/Platform/Emscripten.cmake")` was specified.

```
$ cargo build
   Compiling cc v1.0.73
   Compiling cmake v0.1.48
   Compiling clib-graphviz v0.1.0 (/home/vscode/clib-example/clib-graphviz)
error: failed to run custom build command for `clib-graphviz v0.1.0 (/home/vscode/clib-example/clib-graphviz)`

Caused by:
  process didn't exit successfully: `/home/vscode/clib-example/target/debug/build/clib-graphviz-af4fec6f5c04fc98/build-script-build` (exit status: 101)
  --- stdout
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

  running: "cmake" "/home/vscode/clib-example/clib-graphviz/cpp" 
    "-DCMAKE_BUILD_TYPE=MinSizeRel" 
    "-DCMAKE_TOOLCHAIN_FILE=../emsdk/upstream/emscripten/cmake/Modules/Platform/Emscripten.cmake" 
    "-DCMAKE_INSTALL_PREFIX=/home/vscode/clib-example/target/debug/build/clib-graphviz-8193bad8333c98a0/out" 
    "-DCMAKE_C_FLAGS= -ffunction-sections -fdata-sections -fPIC -m64" 
    "-DCMAKE_CXX_FLAGS= -ffunction-sections -fdata-sections -fPIC -m64" 
    "-DCMAKE_ASM_FLAGS= -ffunction-sections -fdata-sections -fPIC -m64"
  -- Configuring done
  -- Generating done
  -- Build files have been written to: /home/vscode/clib-example/target/debug/build/clib-graphviz-8193bad8333c98a0/out/build
  running: "cmake" "--build" "." "--target" "install" "--config" "Debug" "--parallel" "12"
  Scanning dependencies of target ingraphs
  Scanning dependencies of target cdt
  Scanning dependencies of target label
  Scanning dependencies of target osage
  Scanning dependencies of target pack_obj
  Scanning dependencies of target patchwork
  Scanning dependencies of target ortho
  Scanning dependencies of target fdpgen
  [  0%] Building C object lib/ingraphs/CMakeFiles/ingraphs.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/ingraphs/ingraphs.c.o
  Scanning dependencies of target circogen
  [  0%] Building C object lib/cdt/CMakeFiles/cdt.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/cdt/dtclose.c.o
  [  0%] Building C object lib/label/CMakeFiles/label.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/label/index.c.o
  Scanning dependencies of target dotgen
  [  0%] Building C object lib/osage/CMakeFiles/osage.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/osage/osageinit.c.o
  [  0%] Building C object lib/pack/CMakeFiles/pack_obj.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/pack/ccomps.c.o
  [  0%] Building C object lib/patchwork/CMakeFiles/patchwork.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/patchwork/patchwork.c.o
  [  0%] Building C object lib/ortho/CMakeFiles/ortho.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/ortho/fPQ.c.o
  [  1%] Building C object lib/fdpgen/CMakeFiles/fdpgen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/fdpgen/clusteredges.c.o
  [  1%] Building C object lib/circogen/CMakeFiles/circogen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/circogen/block.c.o
  [  1%] Building C object lib/dotgen/CMakeFiles/dotgen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/dotgen/aspect.c.o
  Scanning dependencies of target common
  Scanning dependencies of target neatogen
  [  1%] Building C object lib/common/CMakeFiles/common.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/common/args.c.o
  [  1%] Building C object lib/neatogen/CMakeFiles/neatogen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/neatogen/adjust.c.o
  [  1%] Building C object lib/cdt/CMakeFiles/cdt.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/cdt/dtdisc.c.o
  [  2%] Linking C static library libingraphs.a
  [  3%] Building C object lib/ortho/CMakeFiles/ortho.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/ortho/maze.c.o
  [  3%] Building C object lib/label/CMakeFiles/label.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/label/node.c.o
  [  4%] Building C object lib/circogen/CMakeFiles/circogen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/circogen/blockpath.c.o
  [  4%] Building C object lib/patchwork/CMakeFiles/patchwork.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/patchwork/patchworkinit.c.o
  [  4%] Building C object lib/fdpgen/CMakeFiles/fdpgen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/fdpgen/comp.c.o
  [  5%] Linking C static library libosage.a
  [  6%] Building C object lib/common/CMakeFiles/common.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/common/arrows.c.o
  [  7%] Building C object lib/pack/CMakeFiles/pack_obj.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/pack/pack.c.o
  [  8%] Building C object lib/dotgen/CMakeFiles/dotgen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/dotgen/acyclic.c.o
  [  9%] Building C object lib/cdt/CMakeFiles/cdt.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/cdt/dtextract.c.o
  [  9%] Built target ingraphs
  [ 10%] Building C object lib/neatogen/CMakeFiles/neatogen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/neatogen/bfs.c.o
  [ 10%] Building C object lib/patchwork/CMakeFiles/patchwork.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/patchwork/tree_map.c.o
  [ 11%] Building C object lib/label/CMakeFiles/label.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/label/rectangle.c.o
  [ 11%] Building C object lib/label/CMakeFiles/label.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/label/split.q.c.o
  [ 12%] Building C object lib/fdpgen/CMakeFiles/fdpgen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/fdpgen/dbg.c.o
  [ 12%] Built target osage
  Scanning dependencies of target pathplan
  [ 12%] Building C object lib/dotgen/CMakeFiles/dotgen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/dotgen/class1.c.o
  [ 12%] Building C object lib/ortho/CMakeFiles/ortho.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/ortho/ortho.c.o
  [ 13%] Building C object lib/pathplan/CMakeFiles/pathplan.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/pathplan/cvt.c.o
  [ 13%] Building C object lib/cdt/CMakeFiles/cdt.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/cdt/dtflatten.c.o
  [ 14%] Building C object lib/circogen/CMakeFiles/circogen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/circogen/blocktree.c.o
  [ 14%] Building C object lib/common/CMakeFiles/common.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/common/colxlate.c.o
  [ 14%] Building C object lib/fdpgen/CMakeFiles/fdpgen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/fdpgen/fdpinit.c.o
  [ 14%] Building C object lib/dotgen/CMakeFiles/dotgen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/dotgen/class2.c.o
  [ 14%] Linking C static library libpatchwork.a
  [ 14%] Building C object lib/neatogen/CMakeFiles/neatogen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/neatogen/call_tri.c.o
  [ 14%] Building C object lib/label/CMakeFiles/label.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/label/xlabels.c.o
  [ 15%] Building C object lib/dotgen/CMakeFiles/dotgen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/dotgen/cluster.c.o
  [ 15%] Building C object lib/pathplan/CMakeFiles/pathplan.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/pathplan/inpoly.c.o
  [ 15%] Building C object lib/cdt/CMakeFiles/cdt.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/cdt/dthash.c.o
  [ 15%] Built target pack_obj
  [ 15%] Building C object lib/circogen/CMakeFiles/circogen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/circogen/circpos.c.o
  Scanning dependencies of target rbtree
  [ 16%] Building C object lib/rbtree/CMakeFiles/rbtree.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/rbtree/misc.c.o
  [ 16%] Built target patchwork
  [ 16%] Building C object lib/neatogen/CMakeFiles/neatogen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/neatogen/circuit.c.o
  [ 16%] Building C object lib/fdpgen/CMakeFiles/fdpgen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/fdpgen/grid.c.o
  [ 16%] Building C object lib/rbtree/CMakeFiles/rbtree.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/rbtree/red_black_tree.c.o
  [ 16%] Building C object lib/dotgen/CMakeFiles/dotgen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/dotgen/compound.c.o
  [ 16%] Building C object lib/pathplan/CMakeFiles/pathplan.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/pathplan/route.c.o
  [ 16%] Building C object lib/common/CMakeFiles/common.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/common/ellipse.c.o
  [ 16%] Building C object lib/ortho/CMakeFiles/ortho.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/ortho/partition.c.o
  [ 17%] Linking C static library liblabel.a
  [ 17%] Building C object lib/rbtree/CMakeFiles/rbtree.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/rbtree/stack.c.o
  [ 18%] Building C object lib/dotgen/CMakeFiles/dotgen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/dotgen/conc.c.o
  [ 19%] Building C object lib/cdt/CMakeFiles/cdt.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/cdt/dtlist.c.o
  [ 19%] Building C object lib/circogen/CMakeFiles/circogen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/circogen/circular.c.o
  [ 20%] Building C object lib/neatogen/CMakeFiles/neatogen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/neatogen/closest.c.o
  [ 21%] Building C object lib/fdpgen/CMakeFiles/fdpgen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/fdpgen/layout.c.o
  [ 22%] Building C object lib/circogen/CMakeFiles/circogen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/circogen/circularinit.c.o
  [ 23%] Linking C static library librbtree.a
  [ 24%] Building C object lib/pathplan/CMakeFiles/pathplan.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/pathplan/shortest.c.o
  [ 24%] Building C object lib/common/CMakeFiles/common.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/common/geom.c.o
  [ 25%] Building C object lib/common/CMakeFiles/common.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/common/emit.c.o
  [ 25%] Built target label
  [ 25%] Building C object lib/pathplan/CMakeFiles/pathplan.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/pathplan/shortestpth.c.o
  [ 25%] Building C object lib/cdt/CMakeFiles/cdt.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/cdt/dtmethod.c.o
  [ 26%] Building C object lib/ortho/CMakeFiles/ortho.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/ortho/rawgraph.c.o
  [ 26%] Building C object lib/dotgen/CMakeFiles/dotgen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/dotgen/decomp.c.o
  Scanning dependencies of target sparse
  [ 27%] Building C object lib/sparse/CMakeFiles/sparse.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/sparse/BinaryHeap.c.o
  [ 27%] Building C object lib/circogen/CMakeFiles/circogen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/circogen/deglist.c.o
  [ 27%] Building C object lib/neatogen/CMakeFiles/neatogen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/neatogen/compute_hierarchy.c.o
  [ 27%] Built target rbtree
  [ 28%] Building C object lib/common/CMakeFiles/common.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/common/globals.c.o
  [ 28%] Building C object lib/pathplan/CMakeFiles/pathplan.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/pathplan/solvers.c.o
  [ 29%] Building C object lib/cdt/CMakeFiles/cdt.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/cdt/dtopen.c.o
  Scanning dependencies of target sfdpgen
  [ 29%] Building C object lib/sfdpgen/CMakeFiles/sfdpgen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/sfdpgen/Multilevel.c.o
  [ 29%] Building C object lib/fdpgen/CMakeFiles/fdpgen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/fdpgen/tlayout.c.o
  [ 29%] Building C object lib/ortho/CMakeFiles/ortho.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/ortho/sgraph.c.o
  [ 29%] Building C object lib/cdt/CMakeFiles/cdt.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/cdt/dtrenew.c.o
  [ 29%] Building C object lib/dotgen/CMakeFiles/dotgen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/dotgen/dotinit.c.o
  [ 30%] Building C object lib/circogen/CMakeFiles/circogen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/circogen/edgelist.c.o
  [ 30%] Building C object lib/sparse/CMakeFiles/sparse.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/sparse/clustering.c.o
  [ 31%] Building C object lib/neatogen/CMakeFiles/neatogen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/neatogen/conjgrad.c.o
  [ 31%] Building C object lib/pathplan/CMakeFiles/pathplan.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/pathplan/triang.c.o
  [ 31%] Building C object lib/sfdpgen/CMakeFiles/sfdpgen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/sfdpgen/post_process.c.o
  [ 32%] Building C object lib/common/CMakeFiles/common.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/common/htmllex.c.o
  [ 32%] Building C object lib/cdt/CMakeFiles/cdt.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/cdt/dtrestore.c.o
  [ 32%] Building C object lib/ortho/CMakeFiles/ortho.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/ortho/trapezoid.c.o
  [ 32%] Building C object lib/circogen/CMakeFiles/circogen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/circogen/nodelist.c.o
  [ 33%] Building C object lib/fdpgen/CMakeFiles/fdpgen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/fdpgen/xlayout.c.o
  [ 33%] Building C object lib/neatogen/CMakeFiles/neatogen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/neatogen/constrained_majorization.c.o
  [ 33%] Building C object lib/sparse/CMakeFiles/sparse.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/sparse/color_palette.c.o
  [ 33%] Building C object lib/pathplan/CMakeFiles/pathplan.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/pathplan/util.c.o
  [ 34%] Building C object lib/dotgen/CMakeFiles/dotgen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/dotgen/dotsplines.c.o
  [ 35%] Building C object lib/sfdpgen/CMakeFiles/sfdpgen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/sfdpgen/PriorityQueue.c.o
  [ 35%] Building C object lib/sfdpgen/CMakeFiles/sfdpgen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/sfdpgen/sfdpinit.c.o
  [ 36%] Building C object lib/cdt/CMakeFiles/cdt.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/cdt/dtsize.c.o
  [ 36%] Building C object lib/circogen/CMakeFiles/circogen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/circogen/nodeset.c.o
  [ 37%] Building C object lib/pathplan/CMakeFiles/pathplan.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/pathplan/visibility.c.o
  [ 37%] Linking C static library libfdpgen.a
  [ 37%] Building C object lib/cdt/CMakeFiles/cdt.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/cdt/dtstat.c.o
  [ 39%] Linking C static library libortho.a
  [ 39%] Building C object lib/sfdpgen/CMakeFiles/sfdpgen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/sfdpgen/sparse_solve.c.o
  [ 39%] Building C object lib/neatogen/CMakeFiles/neatogen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/neatogen/constraint.c.o
  [ 39%] Building C object lib/sfdpgen/CMakeFiles/sfdpgen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/sfdpgen/spring_electrical.c.o
  [ 40%] Building C object lib/sparse/CMakeFiles/sparse.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/sparse/colorutil.c.o
  [ 40%] Building C object lib/sfdpgen/CMakeFiles/sfdpgen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/sfdpgen/stress_model.c.o
  [ 40%] Building C object lib/common/CMakeFiles/common.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/common/htmltable.c.o
  [ 41%] Linking C static library libcircogen.a
  [ 41%] Built target fdpgen
  [ 41%] Linking C static library libpathplan.a
  [ 42%] Building C object lib/sfdpgen/CMakeFiles/sfdpgen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/sfdpgen/uniform_stress.c.o
  [ 43%] Building C object lib/cdt/CMakeFiles/cdt.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/cdt/dtstrhash.c.o
  [ 43%] Built target ortho
  [ 44%] Building C object lib/common/CMakeFiles/common.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/common/input.c.o
  [ 44%] Building C object lib/sparse/CMakeFiles/sparse.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/sparse/DotIO.c.o
  [ 44%] Building C object lib/dotgen/CMakeFiles/dotgen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/dotgen/fastgr.c.o
  [ 45%] Building C object lib/dotgen/CMakeFiles/dotgen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/dotgen/flat.c.o
  [ 46%] Building C object lib/neatogen/CMakeFiles/neatogen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/neatogen/delaunay.c.o
  [ 46%] Built target circogen
  [ 46%] Building C object lib/cdt/CMakeFiles/cdt.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/cdt/dttree.c.o
  Scanning dependencies of target twopigen
  [ 46%] Built target pathplan
  [ 47%] Building C object lib/dotgen/CMakeFiles/dotgen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/dotgen/mincross.c.o
  [ 47%] Building C object lib/twopigen/CMakeFiles/twopigen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/twopigen/circle.c.o
  [ 47%] Building C object lib/twopigen/CMakeFiles/twopigen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/twopigen/twopiinit.c.o
  [ 47%] Building C object lib/neatogen/CMakeFiles/neatogen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/neatogen/dijkstra.c.o
  [ 47%] Building C object lib/dotgen/CMakeFiles/dotgen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/dotgen/position.c.o
  [ 47%] Building C object lib/sparse/CMakeFiles/sparse.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/sparse/general.c.o
  [ 47%] Building C object lib/common/CMakeFiles/common.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/common/intset.c.o
  [ 47%] Building C object lib/cdt/CMakeFiles/cdt.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/cdt/dtview.c.o
  [ 48%] Building C object lib/sparse/CMakeFiles/sparse.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/sparse/IntStack.c.o
  [ 48%] Building C object lib/common/CMakeFiles/common.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/common/labels.c.o
  [ 49%] Building C object lib/cdt/CMakeFiles/cdt.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/cdt/dtwalk.c.o
  [ 50%] Building C object lib/common/CMakeFiles/common.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/common/ns.c.o
  [ 50%] Linking C static library libtwopigen.a
  [ 50%] Linking C static library libsfdpgen.a
  [ 50%] Building C object lib/common/CMakeFiles/common.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/common/memory.c.o
  [ 51%] Building C object lib/common/CMakeFiles/common.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/common/output.c.o
  [ 51%] Building C object lib/sparse/CMakeFiles/sparse.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/sparse/LinkedList.c.o
  [ 52%] Building C object lib/sparse/CMakeFiles/sparse.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/sparse/mq.c.o
  [ 52%] Building C object lib/neatogen/CMakeFiles/neatogen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/neatogen/edges.c.o
  [ 52%] Linking C static library libcdt.a
  [ 52%] Building C object lib/common/CMakeFiles/common.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/common/pointset.c.o
  [ 52%] Built target twopigen
  Scanning dependencies of target vpsc
  [ 53%] Building C object lib/common/CMakeFiles/common.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/common/postproc.c.o
  [ 53%] Building CXX object lib/vpsc/CMakeFiles/vpsc.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/vpsc/block.cpp.o
  [ 53%] Built target sfdpgen
  Scanning dependencies of target xdot
  [ 53%] Building C object lib/xdot/CMakeFiles/xdot.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/xdot/xdot.c.o
  [ 54%] Building C object lib/common/CMakeFiles/common.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/common/psusershape.c.o
  [ 55%] Building C object lib/neatogen/CMakeFiles/neatogen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/neatogen/embed_graph.c.o
  [ 55%] Building C object lib/neatogen/CMakeFiles/neatogen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/neatogen/geometry.c.o
  [ 55%] Built target cdt
  Scanning dependencies of target pack
  [ 55%] Building C object lib/common/CMakeFiles/common.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/common/routespl.c.o
  [ 55%] Linking C static library libpack.a
  [ 56%] Building C object lib/sparse/CMakeFiles/sparse.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/sparse/QuadTree.c.o
  [ 56%] Building C object lib/dotgen/CMakeFiles/dotgen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/dotgen/rank.c.o
  Scanning dependencies of target cgraph
  [ 56%] Building C object lib/cgraph/CMakeFiles/cgraph.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/cgraph/agerror.c.o
  [ 57%] Building C object lib/neatogen/CMakeFiles/neatogen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/neatogen/heap.c.o
  [ 57%] Building C object lib/common/CMakeFiles/common.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/common/shapes.c.o
  [ 57%] Building C object lib/neatogen/CMakeFiles/neatogen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/neatogen/hedges.c.o
  [ 57%] Building C object lib/neatogen/CMakeFiles/neatogen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/neatogen/info.c.o
  [ 58%] Building C object lib/cgraph/CMakeFiles/cgraph.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/cgraph/agxbuf.c.o
  [ 59%] Linking C static library libxdot.a
  [ 59%] Built target pack
  [ 60%] Building C object lib/common/CMakeFiles/common.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/common/splines.c.o
  [ 60%] Building C object lib/common/CMakeFiles/common.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/common/taper.c.o
  [ 61%] Building C object lib/neatogen/CMakeFiles/neatogen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/neatogen/kkutils.c.o
  [ 61%] Building C object lib/sparse/CMakeFiles/sparse.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/sparse/SparseMatrix.c.o
  [ 61%] Building C object lib/neatogen/CMakeFiles/neatogen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/neatogen/legal.c.o
  [ 62%] Building C object lib/neatogen/CMakeFiles/neatogen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/neatogen/lu.c.o
  [ 62%] Building C object lib/cgraph/CMakeFiles/cgraph.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/cgraph/apply.c.o
  [ 62%] Building C object lib/dotgen/CMakeFiles/dotgen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/dotgen/sameport.c.o
  [ 63%] Building C object lib/cgraph/CMakeFiles/cgraph.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/cgraph/attr.c.o
  [ 63%] Built target xdot
  [ 63%] Building C object lib/cgraph/CMakeFiles/cgraph.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/cgraph/edge.c.o
  [ 64%] Building C object lib/cgraph/CMakeFiles/cgraph.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/cgraph/flatten.c.o
  [ 64%] Building C object lib/common/CMakeFiles/common.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/common/textspan.c.o
  [ 64%] Building C object lib/common/CMakeFiles/common.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/common/textspan_lut.c.o
  [ 64%] Building C object lib/neatogen/CMakeFiles/neatogen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/neatogen/matinv.c.o
  [ 64%] Building C object lib/neatogen/CMakeFiles/neatogen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/neatogen/matrix_ops.c.o
  [ 64%] Building C object lib/common/CMakeFiles/common.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/common/timing.c.o
  [ 64%] Linking C static library libdotgen.a
  [ 65%] Building C object lib/cgraph/CMakeFiles/cgraph.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/cgraph/graph.c.o
  [ 65%] Building C object lib/cgraph/CMakeFiles/cgraph.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/cgraph/id.c.o
  [ 66%] Building C object lib/common/CMakeFiles/common.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/common/utils.c.o
  [ 66%] Building C object lib/common/CMakeFiles/common.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/common/xml.c.o
  [ 67%] Building C object lib/cgraph/CMakeFiles/cgraph.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/cgraph/imap.c.o
  [ 68%] Building C object lib/neatogen/CMakeFiles/neatogen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/neatogen/memory.c.o
  [ 69%] Building C object lib/common/CMakeFiles/common.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/build/lib/common/htmlparse.c.o
  [ 69%] Built target dotgen
  [ 69%] Building C object lib/neatogen/CMakeFiles/neatogen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/neatogen/multispline.c.o
  [ 69%] Building CXX object lib/vpsc/CMakeFiles/vpsc.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/vpsc/blocks.cpp.o
  [ 69%] Building C object lib/neatogen/CMakeFiles/neatogen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/neatogen/neatoinit.c.o
  [ 70%] Building C object lib/neatogen/CMakeFiles/neatogen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/neatogen/opt_arrangement.c.o
  [ 70%] Building C object lib/neatogen/CMakeFiles/neatogen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/neatogen/neatosplines.c.o
  [ 71%] Building C object lib/neatogen/CMakeFiles/neatogen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/neatogen/overlap.c.o
  [ 71%] Building C object lib/cgraph/CMakeFiles/cgraph.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/cgraph/io.c.o
  [ 71%] Building C object lib/neatogen/CMakeFiles/neatogen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/neatogen/pca.c.o
  [ 71%] Building C object lib/neatogen/CMakeFiles/neatogen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/neatogen/poly.c.o
  [ 71%] Building CXX object lib/vpsc/CMakeFiles/vpsc.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/vpsc/constraint.cpp.o
  [ 71%] Building C object lib/cgraph/CMakeFiles/cgraph.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/cgraph/mem.c.o
  [ 72%] Building C object lib/sparse/CMakeFiles/sparse.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/sparse/vector.c.o
  [ 73%] Building C object lib/neatogen/CMakeFiles/neatogen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/neatogen/printvis.c.o
  [ 73%] Linking C static library libcommon.a
  [ 73%] Building C object lib/neatogen/CMakeFiles/neatogen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/neatogen/quad_prog_solve.c.o
  [ 75%] Building CXX object lib/vpsc/CMakeFiles/vpsc.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/vpsc/csolve_VPSC.cpp.o
  [ 75%] Building C object lib/neatogen/CMakeFiles/neatogen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/neatogen/site.c.o
  [ 75%] Building C object lib/neatogen/CMakeFiles/neatogen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/neatogen/smart_ini_x.c.o
  [ 76%] Building C object lib/cgraph/CMakeFiles/cgraph.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/cgraph/node.c.o
  [ 76%] Building C object lib/cgraph/CMakeFiles/cgraph.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/cgraph/obj.c.o
  [ 76%] Linking C static library libsparse.a
  [ 76%] Building C object lib/neatogen/CMakeFiles/neatogen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/neatogen/solve.c.o
  [ 76%] Building C object lib/cgraph/CMakeFiles/cgraph.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/cgraph/pend.c.o
  [ 76%] Built target common
  [ 77%] Building C object lib/neatogen/CMakeFiles/neatogen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/neatogen/stuff.c.o
  [ 78%] Building C object lib/cgraph/CMakeFiles/cgraph.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/cgraph/rec.c.o
  [ 78%] Building CXX object lib/vpsc/CMakeFiles/vpsc.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/vpsc/generate-constraints.cpp.o
  [ 78%] Building C object lib/cgraph/CMakeFiles/cgraph.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/cgraph/refstr.c.o
  [ 78%] Built target sparse
  [ 78%] Building C object lib/neatogen/CMakeFiles/neatogen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/neatogen/stress.c.o
  [ 79%] Building C object lib/cgraph/CMakeFiles/cgraph.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/cgraph/subg.c.o
  [ 79%] Building C object lib/neatogen/CMakeFiles/neatogen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/neatogen/voronoi.c.o
  [ 80%] Building C object lib/neatogen/CMakeFiles/neatogen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/neatogen/sgd.c.o
  [ 80%] Building C object lib/cgraph/CMakeFiles/cgraph.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/cgraph/utils.c.o
  [ 80%] Building C object lib/cgraph/CMakeFiles/cgraph.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/cgraph/write.c.o
  [ 81%] Building C object lib/cgraph/CMakeFiles/cgraph.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/build/lib/cgraph/grammar.c.o
  [ 81%] Building C object lib/neatogen/CMakeFiles/neatogen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/neatogen/randomkit.c.o
  [ 82%] Building C object lib/neatogen/CMakeFiles/neatogen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/neatogen/constrained_majorization_ipsep.c.o
  [ 82%] Building C object lib/neatogen/CMakeFiles/neatogen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/neatogen/mosek_quad_solve.c.o
  [ 82%] Building C object lib/cgraph/CMakeFiles/cgraph.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/build/lib/cgraph/scan.c.o
  [ 82%] Building C object lib/neatogen/CMakeFiles/neatogen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/neatogen/quad_prog_vpsc.c.o
  [ 83%] Building CXX object lib/vpsc/CMakeFiles/vpsc.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/vpsc/pairingheap/PairingHeap.cpp.o
  [ 83%] Building CXX object lib/vpsc/CMakeFiles/vpsc.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/vpsc/solve_VPSC.cpp.o
  [ 83%] Building CXX object lib/vpsc/CMakeFiles/vpsc.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/vpsc/variable.cpp.o
  [ 84%] Linking C static library libneatogen.a
  [ 85%] Linking C static library libcgraph.a
  [ 85%] Built target neatogen
  [ 85%] Built target cgraph
  Scanning dependencies of target gvc
  [ 86%] Building C object lib/gvc/CMakeFiles/gvc.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/gvc/gvcontext.c.o
  [ 86%] Building C object lib/gvc/CMakeFiles/gvc.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/gvc/gvconfig.c.o
  [ 86%] Building C object lib/gvc/CMakeFiles/gvc.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/gvc/gvdevice.c.o
  [ 86%] Building C object lib/gvc/CMakeFiles/gvc.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/gvc/gvc.c.o
  [ 86%] Building C object lib/gvc/CMakeFiles/gvc.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/gvc/gvlayout.c.o
  [ 87%] Building C object lib/gvc/CMakeFiles/gvc.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/gvc/gvevent.c.o
  [ 87%] Building C object lib/gvc/CMakeFiles/gvc.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/gvc/gvjobs.c.o
  [ 88%] Building C object lib/gvc/CMakeFiles/gvc.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/gvc/gvloadimage.c.o
  [ 89%] Building C object lib/gvc/CMakeFiles/gvc.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/gvc/gvtextlayout.c.o
  [ 89%] Building C object lib/gvc/CMakeFiles/gvc.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/gvc/gvrender.c.o
  [ 89%] Building C object lib/gvc/CMakeFiles/gvc.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/gvc/gvtool_tred.c.o
  [ 89%] Building C object lib/gvc/CMakeFiles/gvc.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/gvc/gvplugin.c.o
  [ 90%] Building C object lib/gvc/CMakeFiles/gvc.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/gvc/gvusershape.c.o
  [ 90%] Linking C static library libgvc.a
  [ 90%] Built target gvc
  Scanning dependencies of target gvplugin_dot_layout
  Scanning dependencies of target gvplugin_core
  [ 90%] Building C object plugin/dot_layout/CMakeFiles/gvplugin_dot_layout.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/plugin/dot_layout/gvlayout_dot_layout.c.o
  [ 91%] Building C object plugin/dot_layout/CMakeFiles/gvplugin_dot_layout.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/plugin/dot_layout/gvplugin_dot_layout.c.o
  [ 92%] Building C object plugin/core/CMakeFiles/gvplugin_core.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/plugin/core/gvplugin_core.c.o
  [ 92%] Building C object plugin/core/CMakeFiles/gvplugin_core.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/plugin/core/gvrender_core_map.c.o
  [ 92%] Building C object plugin/core/CMakeFiles/gvplugin_core.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/plugin/core/gvloadimage_core.c.o
  [ 93%] Building C object plugin/core/CMakeFiles/gvplugin_core.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/plugin/core/gvrender_core_json.c.o
  [ 93%] Building C object plugin/core/CMakeFiles/gvplugin_core.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/plugin/core/gvrender_core_fig.c.o
  [ 94%] Building C object plugin/core/CMakeFiles/gvplugin_core.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/plugin/core/gvrender_core_mp.c.o
  [ 94%] Building C object plugin/core/CMakeFiles/gvplugin_core.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/plugin/core/gvrender_core_pic.c.o
  [ 94%] Building C object plugin/core/CMakeFiles/gvplugin_core.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/plugin/core/gvrender_core_dot.c.o
  [ 95%] Building C object plugin/core/CMakeFiles/gvplugin_core.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/plugin/core/gvrender_core_pov.c.o
  [ 95%] Building C object plugin/core/CMakeFiles/gvplugin_core.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/plugin/core/gvrender_core_svg.c.o
  [ 95%] Building C object plugin/core/CMakeFiles/gvplugin_core.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/plugin/core/gvrender_core_ps.c.o
  [ 96%] Linking C static library libgvplugin_dot_layout.a
  [ 97%] Building C object plugin/core/CMakeFiles/gvplugin_core.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/plugin/core/gvrender_core_tk.c.o
  [ 97%] Building C object plugin/core/CMakeFiles/gvplugin_core.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/plugin/core/gvrender_core_vml.c.o
  [ 97%] Built target gvplugin_dot_layout
  [ 97%] Linking C static library libgvplugin_core.a
  [ 97%] Built target gvplugin_core
  [ 98%] Linking CXX static library libvpsc.a
  [ 98%] Built target vpsc
  Scanning dependencies of target gvplugin_neato_layout
  [ 98%] Building C object plugin/neato_layout/CMakeFiles/gvplugin_neato_layout.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/plugin/neato_layout/gvplugin_neato_layout.c.o
  [ 98%] Building C object plugin/neato_layout/CMakeFiles/gvplugin_neato_layout.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/plugin/neato_layout/gvlayout_neato_layout.c.o
  [ 99%] Linking CXX static library libgvplugin_neato_layout.a
  [ 99%] Built target gvplugin_neato_layout
  Scanning dependencies of target graphvizlib
  [ 99%] Building CXX object graphvizlib/CMakeFiles/graphvizlib.dir/main.cpp.o
  [100%] Linking CXX executable graphvizlib.js

  --- stderr
  CMake Warning:
    Manually-specified variables were not used by the project:

      CMAKE_ASM_FLAGS


  make: warning: -j12 forced in submake: resetting jobserver mode.
  /home/vscode/clib-example/clib-graphviz/src-graphviz/lib/twopigen/circle.c:354:28: warning: format specifies type 'unsigned long' but the argument has type 'uint64_t' (aka 'unsigned long long') [-Wformat]
                  agnameof(center), maxNStepsToCenter);
                                    ^~~~~~~~~~~~~~~~~
  1 warning generated.
  /home/vscode/clib-example/clib-graphviz/src-graphviz/lib/cgraph/id.c:161:60: warning: format specifies type 'unsigned long' but the argument has type 'IDTYPE' (aka 'unsigned long long') [-Wformat]
          snprintf(buf, sizeof(buf), "%c%" PRIu64, LOCALNAMEPREFIX, AGID(obj));
                                        ~~~~~~~~~                   ^~~~~~~~~
  /home/vscode/clib-example/clib-graphviz/cpp/../src-graphviz/lib/cgraph/cgraph.h:110:20: note: expanded from macro 'AGID'
  #define AGID(obj)               (AGTAG(obj).id)
                                  ^~~~~~~~~~~~~~~
  1 warning generated.
  /home/vscode/clib-example/clib-graphviz/src-graphviz/lib/cgraph/write.c:505:53: warning: format specifies type 'unsigned long' but the argument has type 'IDTYPE' (aka 'unsigned long long') [-Wformat]
          snprintf(buf, sizeof(buf), "_%" PRIu64 "_SUSPECT", AGID(n));    /* could be deadly wrong */
                                       ~~~~~~~~~             ^~~~~~~
  /home/vscode/clib-example/clib-graphviz/cpp/../src-graphviz/lib/cgraph/cgraph.h:110:20: note: expanded from macro 'AGID'
  #define AGID(obj)               (AGTAG(obj).id)
                                  ^~~~~~~~~~~~~~~
  1 warning generated.
  wasm-ld: error: CMakeFiles/graphvizlib.dir/main.cpp.o: must specify -mwasm64 to process wasm64 object files
  em++: error: '/home/vscode/clib-example/clib-graphviz/emsdk/upstream/bin/wasm-ld 
    -o graphvizlib.wasm CMakeFiles/graphvizlib.dir/main.cpp.o 
    ../plugin/core/libgvplugin_core.a 
    ../plugin/dot_layout/libgvplugin_dot_layout.a 
    ../plugin/neato_layout/libgvplugin_neato_layout.a ../lib/common/libcommon.a ../lib/ortho/libortho.a ../lib/pack/libpack.a ../lib/dotgen/libdotgen.a ../lib/gvc/libgvc.a ../lib/pack/libpack.a ../lib/cgraph/libcgraph.a ../lib/cdt/libcdt.a ../lib/label/liblabel.a ../lib/xdot/libxdot.a ../lib/pathplan/libpathplan.a ../lib/circogen/libcircogen.a ../lib/fdpgen/libfdpgen.a ../lib/neatogen/libneatogen.a ../lib/osage/libosage.a ../lib/patchwork/libpatchwork.a ../lib/rbtree/librbtree.a ../lib/sparse/libsparse.a ../lib/twopigen/libtwopigen.a ../lib/vpsc/libvpsc.a ../lib/sfdpgen/libsfdpgen.a 
    -L/home/vscode/clib-example/clib-graphviz/emsdk/upstream/emscripten/cache/sysroot/lib/wasm32-emscripten 
    -lGL -lal -lhtml5 -lstubs -lnoexit -lc -ldlmalloc 
    -lcompiler_rt -lc++-noexcept -lc++abi-noexcept -lsockets 
    -mllvm -combiner-global-alias-analysis=false -mllvm 
    -enable-emscripten-sjlj -mllvm -disable-lsr --import-undefined --strip-debug 
    --export-if-defined=main 
    --export-if-defined=__start_em_asm 
    --export-if-defined=__stop_em_asm 
    --export-if-defined=__main_argc_argv 
    --export=stackSave 
    --export=stackRestore 
    --export=stackAlloc 
    --export=__wasm_call_ctors 
    --export=__errno_location 
    --export=emscripten_builtin_memalign 
    --export=malloc 
    --export=free 
    --export=saveSetjmp 
    --export=setThrew 
    --export=__cxa_is_pointer_type 
    --export-table 
    -z stack-size=5242880 
    --initial-memory=16777216 
    --no-entry 
    --max-memory=16777216 
    --global-base=1024' 
failed (returned 1)
  make[2]: *** [graphvizlib/CMakeFiles/graphvizlib.dir/build.make:111: graphvizlib/graphvizlib.js] Error 1
  make[1]: *** [CMakeFiles/Makefile2:1608: graphvizlib/CMakeFiles/graphvizlib.dir/all] Error 2
  make: *** [Makefile:130: all] Error 2
  thread 'main' panicked at '
  command did not execute successfully, got: exit status: 2

  build script failed, must exit now', /usr/local/cargo/registry/src/github.com-1ecc6299db9ec823/cmake-0.1.48/src/lib.rs:975:5
  note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```