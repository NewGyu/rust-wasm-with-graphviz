#include "main.h"

#include <config.h>
#include <gvc.h>
#include <globals.h>

extern gvplugin_library_t gvplugin_core_LTX_library;
extern gvplugin_library_t gvplugin_dot_layout_LTX_library;
extern gvplugin_library_t gvplugin_neato_layout_LTX_library;

char lastErrorStr[1024];

int vizErrorf(char *buf)
{
    strncpy(lastErrorStr, buf, sizeof(lastErrorStr) - 1);
    return 0;
}
const char *graphviz_version()
{
    return PACKAGE_VERSION;
}

const char *graphviz_lastError()
{
    return lastErrorStr;
}

int YInvert_DEFAULT = 0;
int Nop_DEFAULT = 0;

void graphviz_setYInvert(_Bool yInvert)
{
    Y_invert = yInvert == true ? 1 : YInvert_DEFAULT;
}

void graphviz_setNop(int nop)
{
    Nop = nop > 0 ? nop : Nop_DEFAULT;
}


const char *graphviz_layout(const char *src, const char *format, const char *engine)
{
    lastErrorStr[0] = '\0';

    GVC_t *context = gvContext();
    gvAddLibrary(context, &gvplugin_core_LTX_library);
    gvAddLibrary(context, &gvplugin_dot_layout_LTX_library);
    gvAddLibrary(context, &gvplugin_neato_layout_LTX_library);

    agseterr(AGERR);
    agseterrf(vizErrorf);

    agreadline(1);

    Agraph_t *graph;
    char *result = NULL;
    unsigned int length;
    while ((graph = agmemread(src)))
    {
        if (result == NULL)
        {
            gvLayout(context, graph, engine);
            gvRenderData(context, graph, format, &result, &length);
            gvFreeLayout(context, graph);
        }

        agclose(graph);

        src = "";
    }

    return result;
}
