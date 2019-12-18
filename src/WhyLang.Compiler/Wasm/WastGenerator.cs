using System;
using System.Collections.Generic;
using System.IO;
using System.Text;

namespace WhyLang.Compiler.Wasm
{
    public class WastGenerator
    {
        public void Write(WasmModule module, TextWriter output)
        {
            output.Write("(module)");
        }
    }
}
