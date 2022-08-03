const char *graphviz_version();
const char *graphviz_lastError();
void graphviz_setYInvert(_Bool yInvert);
void graphviz_setNop(int nop);
const char *graphviz_layout(const char *src, const char *format, const char *engine);
