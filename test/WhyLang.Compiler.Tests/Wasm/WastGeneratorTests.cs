using System;
using System.Collections.Generic;
using System.IO;
using System.Text;
using WhyLang.Compiler.Wasm;
using Xunit;

namespace WhyLang.Compiler.Tests.Wasm
{
    public class WastGeneratorTest
    {
        [Fact]
        public void EmptyModule()
        {
            var module = new WasmModule();
            Assert.Equal(
                "(module)",
                GenerateModule(module));
        }

        private string GenerateModule(WasmModule module)
        {
            var generator = new WastGenerator();
            var writer = new StringWriter();
            generator.Write(module, writer);
            return writer.ToString();
        }
    }
}
