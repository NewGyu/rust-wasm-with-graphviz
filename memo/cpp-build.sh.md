```
vscode ➜ ~/clib-example/clib-graphviz (master) $ ./cpp-build.sh 
Adding directories to PATH:
PATH += /home/vscode/clib-example/clib-graphviz/emsdk
PATH += /home/vscode/clib-example/clib-graphviz/emsdk/upstream/emscripten
PATH += /home/vscode/clib-example/clib-graphviz/emsdk/node/14.18.2_64bit/bin

Setting environment variables:
PATH = /home/vscode/clib-example/clib-graphviz/emsdk:/home/vscode/clib-example/clib-graphviz/emsdk/upstream/emscripten:/home/vscode/clib-example/clib-graphviz/emsdk/node/14.18.2_64bit/bin:/home/vscode/emsdk/node/14.18.2_64bit/bin:/home/vscode/.wasmtime/bin:/vscode/vscode-server/bin/linux-x64/4af164ea3a06f701fe3e89a2bcbb421d2026b68f/bin/remote-cli:/usr/local/cargo/bin:/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin:/home/vscode/.local/bin
EMSDK = /home/vscode/clib-example/clib-graphviz/emsdk
EM_CONFIG = /home/vscode/clib-example/clib-graphviz/emsdk/.emscripten
EMSDK_NODE = /home/vscode/clib-example/clib-graphviz/emsdk/node/14.18.2_64bit/bin/node
-- Configuring done
-- Generating done
-- Build files have been written to: /home/vscode/clib-example/clib-graphviz/build
Scanning dependencies of target cdt
[  0%] Building C object lib/cdt/CMakeFiles/cdt.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/cdt/dtclose.c.o
[  0%] Building C object lib/cdt/CMakeFiles/cdt.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/cdt/dtdisc.c.o
[  1%] Building C object lib/cdt/CMakeFiles/cdt.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/cdt/dtextract.c.o
[  1%] Building C object lib/cdt/CMakeFiles/cdt.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/cdt/dtflatten.c.o
[  1%] Building C object lib/cdt/CMakeFiles/cdt.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/cdt/dthash.c.o
[  2%] Building C object lib/cdt/CMakeFiles/cdt.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/cdt/dtlist.c.o
[  2%] Building C object lib/cdt/CMakeFiles/cdt.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/cdt/dtmethod.c.o
[  3%] Building C object lib/cdt/CMakeFiles/cdt.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/cdt/dtopen.c.o
[  3%] Building C object lib/cdt/CMakeFiles/cdt.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/cdt/dtrenew.c.o
[  3%] Building C object lib/cdt/CMakeFiles/cdt.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/cdt/dtrestore.c.o
[  4%] Building C object lib/cdt/CMakeFiles/cdt.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/cdt/dtsize.c.o
[  4%] Building C object lib/cdt/CMakeFiles/cdt.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/cdt/dtstat.c.o
[  5%] Building C object lib/cdt/CMakeFiles/cdt.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/cdt/dtstrhash.c.o
[  5%] Building C object lib/cdt/CMakeFiles/cdt.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/cdt/dttree.c.o
[  5%] Building C object lib/cdt/CMakeFiles/cdt.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/cdt/dtview.c.o
[  6%] Building C object lib/cdt/CMakeFiles/cdt.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/cdt/dtwalk.c.o
[  6%] Linking C static library libcdt.a
[  6%] Built target cdt
Scanning dependencies of target circogen
[  6%] Building C object lib/circogen/CMakeFiles/circogen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/circogen/block.c.o
[  6%] Building C object lib/circogen/CMakeFiles/circogen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/circogen/blockpath.c.o
[  7%] Building C object lib/circogen/CMakeFiles/circogen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/circogen/blocktree.c.o
[  7%] Building C object lib/circogen/CMakeFiles/circogen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/circogen/circpos.c.o
[  7%] Building C object lib/circogen/CMakeFiles/circogen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/circogen/circular.c.o
[  8%] Building C object lib/circogen/CMakeFiles/circogen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/circogen/circularinit.c.o
[  8%] Building C object lib/circogen/CMakeFiles/circogen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/circogen/deglist.c.o
[  9%] Building C object lib/circogen/CMakeFiles/circogen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/circogen/edgelist.c.o
[  9%] Building C object lib/circogen/CMakeFiles/circogen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/circogen/nodelist.c.o
[  9%] Building C object lib/circogen/CMakeFiles/circogen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/circogen/nodeset.c.o
[ 10%] Linking C static library libcircogen.a
[ 10%] Built target circogen
Scanning dependencies of target common
[ 10%] Building C object lib/common/CMakeFiles/common.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/common/args.c.o
[ 11%] Building C object lib/common/CMakeFiles/common.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/common/arrows.c.o
[ 11%] Building C object lib/common/CMakeFiles/common.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/common/colxlate.c.o
[ 11%] Building C object lib/common/CMakeFiles/common.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/common/ellipse.c.o
[ 12%] Building C object lib/common/CMakeFiles/common.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/common/emit.c.o
[ 12%] Building C object lib/common/CMakeFiles/common.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/common/geom.c.o
[ 12%] Building C object lib/common/CMakeFiles/common.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/common/globals.c.o
[ 13%] Building C object lib/common/CMakeFiles/common.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/common/htmllex.c.o
[ 13%] Building C object lib/common/CMakeFiles/common.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/common/htmltable.c.o
[ 14%] Building C object lib/common/CMakeFiles/common.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/common/input.c.o
[ 14%] Building C object lib/common/CMakeFiles/common.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/common/intset.c.o
[ 14%] Building C object lib/common/CMakeFiles/common.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/common/labels.c.o
[ 15%] Building C object lib/common/CMakeFiles/common.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/common/ns.c.o
[ 15%] Building C object lib/common/CMakeFiles/common.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/common/memory.c.o
[ 16%] Building C object lib/common/CMakeFiles/common.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/common/output.c.o
[ 16%] Building C object lib/common/CMakeFiles/common.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/common/pointset.c.o
[ 16%] Building C object lib/common/CMakeFiles/common.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/common/postproc.c.o
[ 17%] Building C object lib/common/CMakeFiles/common.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/common/psusershape.c.o
[ 17%] Building C object lib/common/CMakeFiles/common.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/common/routespl.c.o
[ 17%] Building C object lib/common/CMakeFiles/common.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/common/shapes.c.o
[ 18%] Building C object lib/common/CMakeFiles/common.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/common/splines.c.o
[ 18%] Building C object lib/common/CMakeFiles/common.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/common/taper.c.o
[ 19%] Building C object lib/common/CMakeFiles/common.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/common/textspan.c.o
[ 19%] Building C object lib/common/CMakeFiles/common.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/common/textspan_lut.c.o
[ 19%] Building C object lib/common/CMakeFiles/common.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/common/timing.c.o
[ 20%] Building C object lib/common/CMakeFiles/common.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/common/utils.c.o
[ 20%] Building C object lib/common/CMakeFiles/common.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/common/xml.c.o
[ 21%] Building C object lib/common/CMakeFiles/common.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/build/lib/common/htmlparse.c.o
[ 21%] Linking C static library libcommon.a
[ 21%] Built target common
Scanning dependencies of target dotgen
[ 21%] Building C object lib/dotgen/CMakeFiles/dotgen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/dotgen/aspect.c.o
[ 22%] Building C object lib/dotgen/CMakeFiles/dotgen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/dotgen/acyclic.c.o
[ 22%] Building C object lib/dotgen/CMakeFiles/dotgen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/dotgen/class1.c.o
[ 22%] Building C object lib/dotgen/CMakeFiles/dotgen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/dotgen/class2.c.o
[ 23%] Building C object lib/dotgen/CMakeFiles/dotgen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/dotgen/cluster.c.o
[ 23%] Building C object lib/dotgen/CMakeFiles/dotgen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/dotgen/compound.c.o
[ 24%] Building C object lib/dotgen/CMakeFiles/dotgen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/dotgen/conc.c.o
[ 24%] Building C object lib/dotgen/CMakeFiles/dotgen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/dotgen/decomp.c.o
[ 24%] Building C object lib/dotgen/CMakeFiles/dotgen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/dotgen/dotinit.c.o
[ 25%] Building C object lib/dotgen/CMakeFiles/dotgen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/dotgen/dotsplines.c.o
[ 25%] Building C object lib/dotgen/CMakeFiles/dotgen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/dotgen/fastgr.c.o
[ 26%] Building C object lib/dotgen/CMakeFiles/dotgen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/dotgen/flat.c.o
[ 26%] Building C object lib/dotgen/CMakeFiles/dotgen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/dotgen/mincross.c.o
[ 26%] Building C object lib/dotgen/CMakeFiles/dotgen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/dotgen/position.c.o
[ 27%] Building C object lib/dotgen/CMakeFiles/dotgen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/dotgen/rank.c.o
[ 27%] Building C object lib/dotgen/CMakeFiles/dotgen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/dotgen/sameport.c.o
[ 27%] Linking C static library libdotgen.a
[ 27%] Built target dotgen
Scanning dependencies of target fdpgen
[ 28%] Building C object lib/fdpgen/CMakeFiles/fdpgen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/fdpgen/clusteredges.c.o
[ 28%] Building C object lib/fdpgen/CMakeFiles/fdpgen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/fdpgen/comp.c.o
[ 29%] Building C object lib/fdpgen/CMakeFiles/fdpgen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/fdpgen/dbg.c.o
[ 29%] Building C object lib/fdpgen/CMakeFiles/fdpgen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/fdpgen/fdpinit.c.o
[ 29%] Building C object lib/fdpgen/CMakeFiles/fdpgen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/fdpgen/grid.c.o
[ 30%] Building C object lib/fdpgen/CMakeFiles/fdpgen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/fdpgen/layout.c.o
[ 30%] Building C object lib/fdpgen/CMakeFiles/fdpgen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/fdpgen/tlayout.c.o
[ 31%] Building C object lib/fdpgen/CMakeFiles/fdpgen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/fdpgen/xlayout.c.o
[ 31%] Linking C static library libfdpgen.a
[ 31%] Built target fdpgen
Scanning dependencies of target ingraphs
[ 31%] Building C object lib/ingraphs/CMakeFiles/ingraphs.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/ingraphs/ingraphs.c.o
[ 32%] Linking C static library libingraphs.a
[ 32%] Built target ingraphs
Scanning dependencies of target label
[ 32%] Building C object lib/label/CMakeFiles/label.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/label/index.c.o
[ 32%] Building C object lib/label/CMakeFiles/label.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/label/node.c.o
[ 33%] Building C object lib/label/CMakeFiles/label.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/label/rectangle.c.o
[ 33%] Building C object lib/label/CMakeFiles/label.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/label/split.q.c.o
[ 33%] Building C object lib/label/CMakeFiles/label.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/label/xlabels.c.o
[ 34%] Linking C static library liblabel.a
[ 34%] Built target label
Scanning dependencies of target neatogen
[ 34%] Building C object lib/neatogen/CMakeFiles/neatogen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/neatogen/adjust.c.o
[ 35%] Building C object lib/neatogen/CMakeFiles/neatogen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/neatogen/bfs.c.o
[ 35%] Building C object lib/neatogen/CMakeFiles/neatogen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/neatogen/call_tri.c.o
[ 35%] Building C object lib/neatogen/CMakeFiles/neatogen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/neatogen/circuit.c.o
[ 36%] Building C object lib/neatogen/CMakeFiles/neatogen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/neatogen/closest.c.o
[ 36%] Building C object lib/neatogen/CMakeFiles/neatogen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/neatogen/compute_hierarchy.c.o
[ 37%] Building C object lib/neatogen/CMakeFiles/neatogen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/neatogen/conjgrad.c.o
[ 37%] Building C object lib/neatogen/CMakeFiles/neatogen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/neatogen/constrained_majorization.c.o
[ 37%] Building C object lib/neatogen/CMakeFiles/neatogen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/neatogen/constraint.c.o
[ 38%] Building C object lib/neatogen/CMakeFiles/neatogen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/neatogen/delaunay.c.o
[ 38%] Building C object lib/neatogen/CMakeFiles/neatogen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/neatogen/dijkstra.c.o
[ 38%] Building C object lib/neatogen/CMakeFiles/neatogen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/neatogen/edges.c.o
[ 39%] Building C object lib/neatogen/CMakeFiles/neatogen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/neatogen/embed_graph.c.o
[ 39%] Building C object lib/neatogen/CMakeFiles/neatogen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/neatogen/geometry.c.o
[ 40%] Building C object lib/neatogen/CMakeFiles/neatogen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/neatogen/heap.c.o
[ 40%] Building C object lib/neatogen/CMakeFiles/neatogen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/neatogen/hedges.c.o
[ 40%] Building C object lib/neatogen/CMakeFiles/neatogen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/neatogen/info.c.o
[ 41%] Building C object lib/neatogen/CMakeFiles/neatogen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/neatogen/kkutils.c.o
[ 41%] Building C object lib/neatogen/CMakeFiles/neatogen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/neatogen/legal.c.o
[ 42%] Building C object lib/neatogen/CMakeFiles/neatogen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/neatogen/lu.c.o
[ 42%] Building C object lib/neatogen/CMakeFiles/neatogen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/neatogen/matinv.c.o
[ 42%] Building C object lib/neatogen/CMakeFiles/neatogen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/neatogen/matrix_ops.c.o
[ 43%] Building C object lib/neatogen/CMakeFiles/neatogen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/neatogen/memory.c.o
[ 43%] Building C object lib/neatogen/CMakeFiles/neatogen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/neatogen/multispline.c.o
[ 43%] Building C object lib/neatogen/CMakeFiles/neatogen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/neatogen/neatoinit.c.o
[ 44%] Building C object lib/neatogen/CMakeFiles/neatogen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/neatogen/neatosplines.c.o
[ 44%] Building C object lib/neatogen/CMakeFiles/neatogen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/neatogen/opt_arrangement.c.o
[ 45%] Building C object lib/neatogen/CMakeFiles/neatogen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/neatogen/overlap.c.o
[ 45%] Building C object lib/neatogen/CMakeFiles/neatogen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/neatogen/pca.c.o
[ 45%] Building C object lib/neatogen/CMakeFiles/neatogen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/neatogen/poly.c.o
[ 46%] Building C object lib/neatogen/CMakeFiles/neatogen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/neatogen/printvis.c.o
[ 46%] Building C object lib/neatogen/CMakeFiles/neatogen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/neatogen/quad_prog_solve.c.o
[ 47%] Building C object lib/neatogen/CMakeFiles/neatogen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/neatogen/site.c.o
[ 47%] Building C object lib/neatogen/CMakeFiles/neatogen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/neatogen/smart_ini_x.c.o
[ 47%] Building C object lib/neatogen/CMakeFiles/neatogen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/neatogen/solve.c.o
[ 48%] Building C object lib/neatogen/CMakeFiles/neatogen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/neatogen/stuff.c.o
[ 48%] Building C object lib/neatogen/CMakeFiles/neatogen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/neatogen/stress.c.o
[ 48%] Building C object lib/neatogen/CMakeFiles/neatogen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/neatogen/voronoi.c.o
[ 49%] Building C object lib/neatogen/CMakeFiles/neatogen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/neatogen/sgd.c.o
[ 49%] Building C object lib/neatogen/CMakeFiles/neatogen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/neatogen/randomkit.c.o
[ 50%] Building C object lib/neatogen/CMakeFiles/neatogen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/neatogen/constrained_majorization_ipsep.c.o
[ 50%] Building C object lib/neatogen/CMakeFiles/neatogen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/neatogen/mosek_quad_solve.c.o
[ 50%] Building C object lib/neatogen/CMakeFiles/neatogen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/neatogen/quad_prog_vpsc.c.o
[ 51%] Linking C static library libneatogen.a
[ 51%] Built target neatogen
Scanning dependencies of target ortho
[ 51%] Building C object lib/ortho/CMakeFiles/ortho.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/ortho/fPQ.c.o
[ 52%] Building C object lib/ortho/CMakeFiles/ortho.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/ortho/maze.c.o
[ 52%] Building C object lib/ortho/CMakeFiles/ortho.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/ortho/ortho.c.o
[ 52%] Building C object lib/ortho/CMakeFiles/ortho.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/ortho/partition.c.o
[ 53%] Building C object lib/ortho/CMakeFiles/ortho.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/ortho/rawgraph.c.o
[ 53%] Building C object lib/ortho/CMakeFiles/ortho.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/ortho/sgraph.c.o
[ 53%] Building C object lib/ortho/CMakeFiles/ortho.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/ortho/trapezoid.c.o
[ 54%] Linking C static library libortho.a
[ 54%] Built target ortho
Scanning dependencies of target osage
[ 54%] Building C object lib/osage/CMakeFiles/osage.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/osage/osageinit.c.o
[ 55%] Linking C static library libosage.a
[ 55%] Built target osage
Scanning dependencies of target pack_obj
[ 55%] Building C object lib/pack/CMakeFiles/pack_obj.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/pack/ccomps.c.o
[ 56%] Building C object lib/pack/CMakeFiles/pack_obj.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/pack/pack.c.o
[ 56%] Built target pack_obj
Scanning dependencies of target pack
[ 56%] Linking C static library libpack.a
[ 56%] Built target pack
Scanning dependencies of target patchwork
[ 56%] Building C object lib/patchwork/CMakeFiles/patchwork.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/patchwork/patchwork.c.o
[ 57%] Building C object lib/patchwork/CMakeFiles/patchwork.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/patchwork/patchworkinit.c.o
[ 57%] Building C object lib/patchwork/CMakeFiles/patchwork.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/patchwork/tree_map.c.o
[ 57%] Linking C static library libpatchwork.a
[ 57%] Built target patchwork
Scanning dependencies of target pathplan
[ 58%] Building C object lib/pathplan/CMakeFiles/pathplan.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/pathplan/cvt.c.o
[ 58%] Building C object lib/pathplan/CMakeFiles/pathplan.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/pathplan/inpoly.c.o
[ 58%] Building C object lib/pathplan/CMakeFiles/pathplan.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/pathplan/route.c.o
[ 59%] Building C object lib/pathplan/CMakeFiles/pathplan.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/pathplan/shortest.c.o
[ 59%] Building C object lib/pathplan/CMakeFiles/pathplan.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/pathplan/shortestpth.c.o
[ 60%] Building C object lib/pathplan/CMakeFiles/pathplan.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/pathplan/solvers.c.o
[ 60%] Building C object lib/pathplan/CMakeFiles/pathplan.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/pathplan/triang.c.o
[ 60%] Building C object lib/pathplan/CMakeFiles/pathplan.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/pathplan/util.c.o
[ 61%] Building C object lib/pathplan/CMakeFiles/pathplan.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/pathplan/visibility.c.o
[ 61%] Linking C static library libpathplan.a
[ 61%] Built target pathplan
Scanning dependencies of target rbtree
[ 62%] Building C object lib/rbtree/CMakeFiles/rbtree.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/rbtree/misc.c.o
[ 62%] Building C object lib/rbtree/CMakeFiles/rbtree.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/rbtree/red_black_tree.c.o
[ 62%] Building C object lib/rbtree/CMakeFiles/rbtree.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/rbtree/stack.c.o
[ 63%] Linking C static library librbtree.a
[ 63%] Built target rbtree
Scanning dependencies of target sparse
[ 64%] Building C object lib/sparse/CMakeFiles/sparse.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/sparse/BinaryHeap.c.o
[ 64%] Building C object lib/sparse/CMakeFiles/sparse.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/sparse/clustering.c.o
[ 64%] Building C object lib/sparse/CMakeFiles/sparse.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/sparse/color_palette.c.o
[ 65%] Building C object lib/sparse/CMakeFiles/sparse.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/sparse/colorutil.c.o
[ 65%] Building C object lib/sparse/CMakeFiles/sparse.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/sparse/DotIO.c.o
[ 65%] Building C object lib/sparse/CMakeFiles/sparse.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/sparse/general.c.o
[ 66%] Building C object lib/sparse/CMakeFiles/sparse.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/sparse/IntStack.c.o
[ 66%] Building C object lib/sparse/CMakeFiles/sparse.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/sparse/LinkedList.c.o
[ 67%] Building C object lib/sparse/CMakeFiles/sparse.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/sparse/mq.c.o
[ 67%] Building C object lib/sparse/CMakeFiles/sparse.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/sparse/QuadTree.c.o
[ 67%] Building C object lib/sparse/CMakeFiles/sparse.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/sparse/SparseMatrix.c.o
[ 68%] Building C object lib/sparse/CMakeFiles/sparse.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/sparse/vector.c.o
[ 68%] Linking C static library libsparse.a
[ 68%] Built target sparse
Scanning dependencies of target sfdpgen
[ 68%] Building C object lib/sfdpgen/CMakeFiles/sfdpgen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/sfdpgen/Multilevel.c.o
[ 68%] Building C object lib/sfdpgen/CMakeFiles/sfdpgen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/sfdpgen/post_process.c.o
[ 69%] Building C object lib/sfdpgen/CMakeFiles/sfdpgen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/sfdpgen/PriorityQueue.c.o
[ 69%] Building C object lib/sfdpgen/CMakeFiles/sfdpgen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/sfdpgen/sfdpinit.c.o
[ 70%] Building C object lib/sfdpgen/CMakeFiles/sfdpgen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/sfdpgen/sparse_solve.c.o
[ 70%] Building C object lib/sfdpgen/CMakeFiles/sfdpgen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/sfdpgen/spring_electrical.c.o
[ 70%] Building C object lib/sfdpgen/CMakeFiles/sfdpgen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/sfdpgen/stress_model.c.o
[ 71%] Building C object lib/sfdpgen/CMakeFiles/sfdpgen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/sfdpgen/uniform_stress.c.o
[ 71%] Linking C static library libsfdpgen.a
[ 71%] Built target sfdpgen
Scanning dependencies of target twopigen
[ 72%] Building C object lib/twopigen/CMakeFiles/twopigen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/twopigen/circle.c.o
[ 72%] Building C object lib/twopigen/CMakeFiles/twopigen.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/twopigen/twopiinit.c.o
[ 72%] Linking C static library libtwopigen.a
[ 72%] Built target twopigen
Scanning dependencies of target vpsc
[ 73%] Building CXX object lib/vpsc/CMakeFiles/vpsc.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/vpsc/block.cpp.o
[ 73%] Building CXX object lib/vpsc/CMakeFiles/vpsc.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/vpsc/blocks.cpp.o
[ 73%] Building CXX object lib/vpsc/CMakeFiles/vpsc.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/vpsc/constraint.cpp.o
[ 74%] Building CXX object lib/vpsc/CMakeFiles/vpsc.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/vpsc/csolve_VPSC.cpp.o
[ 74%] Building CXX object lib/vpsc/CMakeFiles/vpsc.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/vpsc/generate-constraints.cpp.o
[ 75%] Building CXX object lib/vpsc/CMakeFiles/vpsc.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/vpsc/pairingheap/PairingHeap.cpp.o
[ 75%] Building CXX object lib/vpsc/CMakeFiles/vpsc.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/vpsc/solve_VPSC.cpp.o
[ 75%] Building CXX object lib/vpsc/CMakeFiles/vpsc.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/vpsc/variable.cpp.o
[ 76%] Linking CXX static library libvpsc.a
[ 76%] Built target vpsc
Scanning dependencies of target xdot
[ 76%] Building C object lib/xdot/CMakeFiles/xdot.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/xdot/xdot.c.o
[ 77%] Linking C static library libxdot.a
[ 77%] Built target xdot
Scanning dependencies of target cgraph
[ 77%] Building C object lib/cgraph/CMakeFiles/cgraph.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/cgraph/agerror.c.o
[ 78%] Building C object lib/cgraph/CMakeFiles/cgraph.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/cgraph/agxbuf.c.o
[ 78%] Building C object lib/cgraph/CMakeFiles/cgraph.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/cgraph/apply.c.o
[ 79%] Building C object lib/cgraph/CMakeFiles/cgraph.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/cgraph/attr.c.o
[ 79%] Building C object lib/cgraph/CMakeFiles/cgraph.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/cgraph/edge.c.o
[ 79%] Building C object lib/cgraph/CMakeFiles/cgraph.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/cgraph/flatten.c.o
[ 80%] Building C object lib/cgraph/CMakeFiles/cgraph.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/cgraph/graph.c.o
[ 80%] Building C object lib/cgraph/CMakeFiles/cgraph.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/cgraph/id.c.o
[ 81%] Building C object lib/cgraph/CMakeFiles/cgraph.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/cgraph/imap.c.o
[ 81%] Building C object lib/cgraph/CMakeFiles/cgraph.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/cgraph/io.c.o
[ 81%] Building C object lib/cgraph/CMakeFiles/cgraph.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/cgraph/mem.c.o
[ 82%] Building C object lib/cgraph/CMakeFiles/cgraph.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/cgraph/node.c.o
[ 82%] Building C object lib/cgraph/CMakeFiles/cgraph.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/cgraph/obj.c.o
[ 82%] Building C object lib/cgraph/CMakeFiles/cgraph.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/cgraph/pend.c.o
[ 83%] Building C object lib/cgraph/CMakeFiles/cgraph.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/cgraph/rec.c.o
[ 83%] Building C object lib/cgraph/CMakeFiles/cgraph.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/cgraph/refstr.c.o
[ 84%] Building C object lib/cgraph/CMakeFiles/cgraph.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/cgraph/subg.c.o
[ 84%] Building C object lib/cgraph/CMakeFiles/cgraph.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/cgraph/utils.c.o
[ 84%] Building C object lib/cgraph/CMakeFiles/cgraph.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/cgraph/write.c.o
[ 85%] Building C object lib/cgraph/CMakeFiles/cgraph.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/build/lib/cgraph/grammar.c.o
[ 85%] Building C object lib/cgraph/CMakeFiles/cgraph.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/build/lib/cgraph/scan.c.o
[ 86%] Linking C static library libcgraph.a
[ 86%] Built target cgraph
Scanning dependencies of target gvc
[ 86%] Building C object lib/gvc/CMakeFiles/gvc.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/gvc/gvc.c.o
[ 86%] Building C object lib/gvc/CMakeFiles/gvc.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/gvc/gvconfig.c.o
[ 87%] Building C object lib/gvc/CMakeFiles/gvc.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/gvc/gvcontext.c.o
[ 87%] Building C object lib/gvc/CMakeFiles/gvc.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/gvc/gvdevice.c.o
[ 88%] Building C object lib/gvc/CMakeFiles/gvc.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/gvc/gvevent.c.o
[ 88%] Building C object lib/gvc/CMakeFiles/gvc.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/gvc/gvjobs.c.o
[ 88%] Building C object lib/gvc/CMakeFiles/gvc.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/gvc/gvlayout.c.o
[ 89%] Building C object lib/gvc/CMakeFiles/gvc.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/gvc/gvloadimage.c.o
[ 89%] Building C object lib/gvc/CMakeFiles/gvc.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/gvc/gvplugin.c.o
[ 90%] Building C object lib/gvc/CMakeFiles/gvc.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/gvc/gvrender.c.o
[ 90%] Building C object lib/gvc/CMakeFiles/gvc.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/gvc/gvtextlayout.c.o
[ 90%] Building C object lib/gvc/CMakeFiles/gvc.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/gvc/gvtool_tred.c.o
[ 91%] Building C object lib/gvc/CMakeFiles/gvc.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/lib/gvc/gvusershape.c.o
[ 91%] Linking C static library libgvc.a
[ 91%] Built target gvc
Scanning dependencies of target gvplugin_core
[ 91%] Building C object plugin/core/CMakeFiles/gvplugin_core.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/plugin/core/gvloadimage_core.c.o
[ 92%] Building C object plugin/core/CMakeFiles/gvplugin_core.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/plugin/core/gvplugin_core.c.o
[ 92%] Building C object plugin/core/CMakeFiles/gvplugin_core.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/plugin/core/gvrender_core_dot.c.o
[ 93%] Building C object plugin/core/CMakeFiles/gvplugin_core.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/plugin/core/gvrender_core_json.c.o
[ 93%] Building C object plugin/core/CMakeFiles/gvplugin_core.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/plugin/core/gvrender_core_fig.c.o
[ 93%] Building C object plugin/core/CMakeFiles/gvplugin_core.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/plugin/core/gvrender_core_map.c.o
[ 94%] Building C object plugin/core/CMakeFiles/gvplugin_core.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/plugin/core/gvrender_core_mp.c.o
[ 94%] Building C object plugin/core/CMakeFiles/gvplugin_core.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/plugin/core/gvrender_core_pic.c.o
[ 95%] Building C object plugin/core/CMakeFiles/gvplugin_core.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/plugin/core/gvrender_core_pov.c.o
[ 95%] Building C object plugin/core/CMakeFiles/gvplugin_core.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/plugin/core/gvrender_core_ps.c.o
[ 95%] Building C object plugin/core/CMakeFiles/gvplugin_core.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/plugin/core/gvrender_core_svg.c.o
[ 96%] Building C object plugin/core/CMakeFiles/gvplugin_core.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/plugin/core/gvrender_core_tk.c.o
[ 96%] Building C object plugin/core/CMakeFiles/gvplugin_core.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/plugin/core/gvrender_core_vml.c.o
[ 96%] Linking C static library libgvplugin_core.a
[ 96%] Built target gvplugin_core
Scanning dependencies of target gvplugin_dot_layout
[ 97%] Building C object plugin/dot_layout/CMakeFiles/gvplugin_dot_layout.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/plugin/dot_layout/gvplugin_dot_layout.c.o
[ 97%] Building C object plugin/dot_layout/CMakeFiles/gvplugin_dot_layout.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/plugin/dot_layout/gvlayout_dot_layout.c.o
[ 98%] Linking C static library libgvplugin_dot_layout.a
[ 98%] Built target gvplugin_dot_layout
Scanning dependencies of target gvplugin_neato_layout
[ 98%] Building C object plugin/neato_layout/CMakeFiles/gvplugin_neato_layout.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/plugin/neato_layout/gvplugin_neato_layout.c.o
[ 98%] Building C object plugin/neato_layout/CMakeFiles/gvplugin_neato_layout.dir/home/vscode/clib-example/clib-graphviz/src-graphviz/plugin/neato_layout/gvlayout_neato_layout.c.o
[ 99%] Linking CXX static library libgvplugin_neato_layout.a
[ 99%] Built target gvplugin_neato_layout
Scanning dependencies of target graphvizlib
[ 99%] Building CXX object graphvizlib/CMakeFiles/graphvizlib.dir/main.cpp.o
[100%] Linking CXX executable graphvizlib.js
[100%] Built target graphvizlib
```