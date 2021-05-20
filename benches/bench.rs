use criterion::{black_box, criterion_group, criterion_main, Criterion, Throughput};
use std::sync::Arc;

fn bench_script(c: &mut Criterion) {
    let mut group = c.benchmark_group("script");
    group.throughput(Throughput::Elements(1));
    /*goup.bench_function("gluon_cold", |b| {
        let vm = gluon::new_vm();
        let mut i = 0_u64;
        b.iter(|| {
            //TODO: push i into VM?
            vm.run_expr::<bool>("even", "(i & 1) == 0").unwrap()
        })
    });
    */
    group.bench_function("rhai_cold", |b| {
        let engine = rhai::Engine::new();
        let mut scope = rhai::Scope::new();
        let mut i = 0_u64;
        b.iter(|| {
            scope.push("i", black_box(i));
            i = i.wrapping_add(1);
            engine.eval_with_scope::<bool>(&mut scope, "i % 2 == 0")
        })
    });
    group.bench_function("rhai_preparsed", |b| {
        let engine = rhai::Engine::new();
        let mut scope = rhai::Scope::new();
        let expr = engine.compile_expression("i % 2 == 0").unwrap();
        let mut i = 0_u64;
        b.iter(|| {
            scope.push("i", black_box(i));
            i = i.wrapping_add(1);
            engine.eval_ast_with_scope::<bool>(&mut scope, &expr)
        })
    });
    group.bench_function("rhai_fun", |b| {
        let engine = rhai::Engine::new();
        let ast = engine.compile("fn even(i) { i % 2 == 0 }").unwrap();
        let mut scope = rhai::Scope::new();
        //let ast = engine.optimize_ast(&mut scope, ast, rhai::OptimizationLevel::Full);
        let mut i = 0_i64;
        b.iter(|| {
            let result: bool = engine.call_fn(&mut scope, &ast, "even", (black_box(i),)).unwrap();
            i = i.wrapping_add(1);
            result
        })
    });
    group.bench_function("dyon_preparsed", |b| {
        let mut runtime = dyon::Runtime::new();
        let mut module = dyon::Module::new();
        dyon::load_str(
            "main.rs",
            Arc::new(
                "fn even(x) -> bool { if (x % 2) == 0 { return true } else { return false } }"
                    .into(),
            ),
            &mut module,
        )
        .unwrap();
        let module = Arc::new(module);
        let mut i = 0_f64;
        b.iter(|| {
            let res = runtime
                .call_str_ret("even", &[dyon::Variable::F64(black_box(i), None)], &module)
                .unwrap();
            i += 1.0;
            res
        })
    });
    // gluon doesn't even compile, so I removed it
    group.bench_function("hlua_cold", |b| {
        let mut lua = hlua::Lua::new();
        let mut i = 0_f64;
        b.iter(|| {
            lua.set("i", black_box(i));
            i += 1.0;
            lua.execute::<bool>("return (i % 2) == 0").unwrap()
        })
    });
    group.bench_function("hlua_fun", |b| {
        let mut lua = hlua::Lua::new();
        let mut i = 0_f64;
        lua.execute::<()>("function even(x) return (x % 2) == 0 end").unwrap();
        b.iter(|| {
            i += 1.0;
            let mut fun: hlua::LuaFunction<_> = lua.get("even").unwrap();
            let result: bool = fun.call_with_args((black_box(i),)).unwrap();
            result
        })
    });
    group.bench_function("rlua_cold", |b| {
        let lua = rlua::Lua::new();
        let mut i = 0_f64;
        lua.context(|ctx| {
            let globals = ctx.globals();
            b.iter(|| {
                globals.set("i", black_box(i)).unwrap();
                i += 1.0;
                ctx.load("return (i % 2) == 0").eval::<bool>()
            })
        })
    });
    group.bench_function("rlua_fun", |b| {
        let lua = rlua::Lua::new();
        let mut i = 0_f64;
        lua.context(|ctx| {
            let even = ctx.load("function even(x) return x & 1 == 0 end").into_function().unwrap();
            b.iter(|| {
                i += 1.0;
                let result: bool = even.call((black_box(i),)).unwrap();
                result
            })
        })
    });
    group.bench_function("purua", |b| {
        use combine::EasyParser as _;

        let mut l = purua::state::LuaState::new(4096);
        purua::prelude::prelude(&mut l);
        let mut parser = (combine::parser::char::spaces(), purua::parser::chunk());
        let pos = combine::stream::position::Stream::new("function even(x) return x end");
        let chunk = parser.easy_parse(pos).unwrap().0 .1;
        purua::eval::eval_chunk(&mut l, chunk.as_ref()).unwrap();
        let mut i = 0_i64;
        let even = l.get_global("even").unwrap();
        b.iter(|| {
            let result = l.funcall(even.clone(), vec![purua::value::Value::Number(i)]).unwrap();
            i += 1;
            result
        })
    });
    group.bench_function("koto", |b| {
        use koto::{Koto, runtime::{Value, ValueNumber}};
        let mut koto = Koto::new();
        let chunk = koto.compile("|n| n % 2 == 0").unwrap();
        let even = koto.run_chunk(chunk).unwrap();
        let mut i = 0_i64;
        b.iter(|| {
            let result = koto.call_function(even.clone(), &[Value::Number(ValueNumber::I64(i))]).unwrap();
            i += 1;
            result
        })
    });
    group.finish();
}

criterion_group!(benches, bench_script);
criterion_main!(benches);
