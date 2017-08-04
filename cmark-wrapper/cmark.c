#include <errno.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#define ARRAY_SIZE(x) (sizeof(x)/sizeof(x[0]))
#if 0
#include "../build/src/config.h"
#include "../cmark/extensions/core-extensions.h"
#include "../cmark/src/cmark.h"
#include "../cmark/src/cmark_extension_api.h"
#include "../cmark/src/node.h"
#include "../cmark/src/parser.h"
#include "../cmark/src/registry.h"
#include "../cmark/src/syntax_extension.h"
#endif
#include "cmark.h"
#include "cmark_extension_api.h"
#include "config.h"
#include "core-extensions.h"
#include "node.h"
#include "parser.h"
#include "registry.h"
#include "syntax_extension.h"

#define __FILENAME__                                                           \
    (strrchr(__FILE__, '/') ? strrchr(__FILE__, '/') + 1 : __FILE__)
#if 1
#define LOGD(fmt, ...)                                                         \
    do {                                                                       \
        fprintf(stderr, "%s(%3d) [D]: " fmt "\n", __FILENAME__, __LINE__,      \
                ##__VA_ARGS__);                                                \
    } while (0)
#define LOGE(fmt, ...)                                                         \
    do {                                                                       \
        fprintf(stderr, "%s(%3d) [E]: " fmt "\n", __FILENAME__, __LINE__,      \
                ##__VA_ARGS__);                                                \
    } while (0)
#define LOGW(fmt, ...)                                                         \
    do {                                                                       \
        printf("%s(%3d) [W]: " fmt "\n", __FILENAME__, __LINE__,               \
               ##__VA_ARGS__);                                                 \
    } while (0)
#define LOGI(fmt, ...)                                                         \
    do {                                                                       \
        printf("%s(%3d) [I]: " fmt "\n", __FILENAME__, __LINE__,               \
               ##__VA_ARGS__);                                                 \
    } while (0)
#else
#define LOGD(...)
#define LOGE(fmt, ...)                                                         \
    do {                                                                       \
        printf("%s(%3d) [E]: " fmt "\n", __FILENAME__, __LINE__,               \
               ##__VA_ARGS__);                                                 \
    } while (0)
#define LOGW(fmt, ...)                                                         \
    do {                                                                       \
        printf("%s(%3d) [W]: " fmt "\n", __FILENAME__, __LINE__,               \
               ##__VA_ARGS__);                                                 \
    } while (0)
#define LOGI(fmt, ...)                                                         \
    do {                                                                       \
        printf("%s(%3d) [I]: " fmt "\n", __FILENAME__, __LINE__,               \
               ##__VA_ARGS__);                                                 \
    } while (0)
#endif

typedef enum {
    FORMAT_NONE,
    FORMAT_HTML,
    FORMAT_XML,
    FORMAT_MAN,
    FORMAT_COMMONMARK,
    FORMAT_PLAINTEXT,
    FORMAT_LATEX
} writer_format;

static bool print_document(cmark_node *document, writer_format writer,
                           int options, int width, cmark_parser *parser) {
    char *result;

    cmark_mem *mem = cmark_get_default_mem_allocator();

    switch (writer) {
    case FORMAT_HTML:
        result = cmark_render_html_with_mem(document, options,
                                            parser->syntax_extensions, mem);
        break;
    case FORMAT_XML:
        result = cmark_render_xml_with_mem(document, options, mem);
        break;
    case FORMAT_MAN:
        result = cmark_render_man_with_mem(document, options, width, mem);
        break;
    case FORMAT_COMMONMARK:
        result =
            cmark_render_commonmark_with_mem(document, options, width, mem);
        break;
    case FORMAT_PLAINTEXT:
        result = cmark_render_plaintext_with_mem(document, options, width, mem);
        break;
    case FORMAT_LATEX:
        result = cmark_render_latex_with_mem(document, options, width, mem);
        break;
    default:
        LOGE("Unknown format %d", writer);
        return false;
    }
    LOGD("%s", result);
    mem->free(result);

    return true;
}

static void print_commonmark_extensions(void) {
    cmark_llist *syntax_extensions;
    cmark_llist *tmp;
    cmark_mem *mem = cmark_get_default_mem_allocator();
    syntax_extensions = cmark_list_syntax_extensions(mem);
    for (tmp = syntax_extensions; tmp; tmp = tmp->next) {
        cmark_syntax_extension *ext = (cmark_syntax_extension *)tmp->data;
        LOGI("%s", ext->name);
    }

    cmark_llist_free(mem, syntax_extensions);
}

char *commonmark_parser(const char *buffer, size_t bytes) {
    cmark_parser *parser = NULL;
    cmark_node *document = NULL;
    int width = 0;
    char *unparsed;
    writer_format writer = FORMAT_HTML;
    int options = CMARK_OPT_DEFAULT;
    char *result = NULL;

    cmark_register_plugin(core_extensions_registration);
    // print_commonmark_extensions();
    // #if defined(_WIN32) && !defined(__CYGWIN__)
    //   _setmode(_fileno(stdin), _O_BINARY);
    //   _setmode(_fileno(stdout), _O_BINARY);
    // #endif

    //"--sourcepos") == 0) {
    //  options |= CMARK_OPT_SOURCEPOS;
    //"--hardbreaks") == 0) {
    //  options |= CMARK_OPT_HARDBREAKS;
    //"--nobreaks") == 0) {
    //  options |= CMARK_OPT_NOBREAKS;
    //"--smart") == 0) {
    options |= CMARK_OPT_SMART;
    // "--github-pre-lang") == 0) {
    options |= CMARK_OPT_GITHUB_PRE_LANG;
//"--safe") == 0) {
//  options |= CMARK_OPT_SAFE;
//"--validate-utf8") == 0) {
//  options |= CMARK_OPT_VALIDATE_UTF8;
//"--liberal-html-tag") == 0) {
//  options |= CMARK_OPT_LIBERAL_HTML_TAG;

#if DEBUG
    parser = cmark_parser_new(options);
#else
    parser =
        cmark_parser_new_with_mem(options, cmark_get_arena_mem_allocator());
#endif
    if (!parser) {
        cmark_release_plugins();
        LOGE("cmark_parser_new failed");
        return result; // Error
    }
    const char *ext[] = {"table", "strikethrough", "tagfilter", "autolink"};
	// LOGD("ext length %d", sizeof(ext)/sizeof(*ext));
    int i = 0;
    for (i = 0; i < sizeof(ext)/sizeof(*ext); i++) {
        cmark_syntax_extension *syntax_extension =
            cmark_find_syntax_extension(ext[i]);
        if (syntax_extension)
            cmark_parser_attach_syntax_extension(parser, syntax_extension);
    }

    cmark_parser_feed(parser, buffer, bytes);
    document = cmark_parser_finish(parser);
    if (document) {
        cmark_mem *mem = cmark_get_default_mem_allocator();
        result = cmark_render_html_with_mem(document, options,
                                            parser->syntax_extensions, mem);
        // user to free result by mem->free(xxx)
        // mem->free(result);
    } else {
        LOGE("cmark_parser_finish failed");
        result = NULL; // Error
    }

#if DEBUG
    if (parser) {
        cmark_parser_free(parser);
    }

    if (document) {
        cmark_node_free(document);
    }
#else
    cmark_arena_reset();
#endif

    cmark_release_plugins();
    return result;
}

#if 0
static const char base_html[] = {
#include "base.html.inc"
    , 0};
static const char base_css[] = {
#include "base.css.inc"
    , 0};

bool render_markdown_to_html(const std::string &in, std::string &out) {
    char *result = commonmark_parser(in.c_str(), in.size());
    if (result) {
        std::string css(base_css);
        out = std::string(base_html);
        std::string body(result);
        cmark_mem *mem = cmark_get_default_mem_allocator();
        std::regex CSSLabel("\\{%style%\\}");
        std::regex BodyLabel("\\{%body%\\}");
        out = std::regex_replace(out, CSSLabel, css); // insert CSS stylesheet
        out = std::regex_replace(out, BodyLabel,
                                 body); // insert HTML of markdown
        LOGD("\n%s", out.c_str());
        mem->free(result);
        return true;
    }
    return false;
}
#endif

unsigned int markdown_to_html(const char *markdown, char** html) {
    *html = commonmark_parser(markdown, strlen(markdown));
	return strlen(*html);
}

void markdown_free(char *html) {
    cmark_mem *mem = cmark_get_default_mem_allocator();
    if (mem && html) {
        mem->free(html);
    }
}
