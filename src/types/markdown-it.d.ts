declare module 'markdown-it' {
  interface MarkdownItOptions {
    html?: boolean
    breaks?: boolean
    linkify?: boolean
    typographer?: boolean
    quotes?: string
    xhtmlOut?: boolean
    langPrefix?: string
    highlight?: (str: string, lang: string) => string
  }

  class MarkdownIt {
    constructor(options?: MarkdownItOptions)
    render(markdown: string): string
    renderInline(markdown: string): string
    parse(markdown: string, env?: any): any[]
    parseInline(markdown: string, env?: any): any[]
    use(plugin: any, ...args: any[]): MarkdownIt
    configure(presets: string | MarkdownItOptions): MarkdownIt
    set(options: MarkdownItOptions): MarkdownIt
    enable(list: string | string[], ignoreInvalid?: boolean): MarkdownIt
    disable(list: string | string[], ignoreInvalid?: boolean): MarkdownIt
  }

  const MarkdownIt: {
    new (options?: MarkdownItOptions): MarkdownIt
    (): MarkdownIt
  }

  export = MarkdownIt
}
