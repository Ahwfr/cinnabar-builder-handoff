use anyhow::{Result, ensure};
use wasmtime::{Config, Engine, Store};
use wasmtime::component::{Component, Linker, Val};
use Val::*;

fn rec<const N: usize>(fields: [(&str, Val); N]) -> Val {
    Record(fields.into_iter().map(|(name, v)| (name.into(), v)).collect())
}
fn s(text: &str) -> Val { String(text.into()) }
fn some(v: Val) -> Val { Option(Some(Box::new(v))) }
fn ok(v: Val) -> Val { Result(Ok(Some(Box::new(v)))) }
fn unit() -> Val { Result(Ok(None)) }
fn field<'a>(v: &'a Val, name: &str) -> &'a Val {
    let Record(fields) = v else { panic!("expected record") };
    &fields.iter().find(|(n, _)| n == name).unwrap().1
}
fn i(v: &Val) -> i32 { let S32(v) = v else { panic!("expected integer") }; *v }

#[derive(Default)]
struct State {
    key: &'static str,
    label: std::string::String,
    dimension: i32,
    reject_grid: bool,
    next_handle: u64,
    grids: u32,
    meshes: u32,
    removes: u32,
    calls: u32,
    active: std::collections::BTreeSet<u64>,
    saved: u32,
    panel: bool,
    pages: u32,
    event: &'static str,
    max_draws: usize,
}
fn main() -> Result<()> {
    let path = std::env::args_os().nth(1).unwrap();
    let stress = true;

    let engine = Engine::new(Config::new().consume_fuel(true))?;
    let component = Component::from_file(&engine, path)?;
    let mut linker = Linker::<State>::new(&engine);
    linker.instance("cinnabar:extension/hud@0.1.0")?.func_new("set-label", |mut cx, _, args, out| {
        let String(label) = &args[0] else { unreachable!() };
        cx.data_mut().label = label.clone();
        out[0] = unit();
        Ok(())
    })?;
    linker.instance("cinnabar:extension/gameplay@0.1.0")?.func_new("read-frame", |cx, _, _, out| {
        out[0] = ok(some(rec([
            ("session", U64(1)), ("dimension", S32(cx.data().dimension)),
            ("eye", rec([("x", Float32(0.)), ("y", Float32(65.62)), ("z", Float32(0.))])),
            ("yaw", Float32(0.)), ("pitch", Float32(0.)),
            ("frame-seconds", Float32(0.017)), ("players", List(vec![])),
        ])));
        Ok(())
    })?;
    let mut input = linker.instance("cinnabar:extension/input@0.1.0")?;
    input.func_new("read-controls", |cx, _, _, out| {
        let key = cx.data().key;
        out[0] = ok(rec([
            ("seconds", Float32(0.)), ("focused", Bool(true)), ("gameplay", Bool(true)),
            ("panel-open", Bool(cx.data().panel)),
            ("keys-pressed", List(if key.is_empty() {vec![]} else {vec![s(key)]})),
            ("keys-held", List(vec![s("AltLeft")])), ("keys-admitted", List(vec![])),
            ("events", List(if cx.data().event.is_empty(){vec![]}else{vec![rec([("id",s(cx.data().event)),("value",Float32(1.))])]})), ("pointer", Option(None)), ("wheel", Float32(0.)),
        ]));
        Ok(())
    })?;
    input.func_new("read-text-events", |_, _, _, out| {
        out[0] = ok(List(vec![]));
        Ok(())
    })?;
    linker.instance("cinnabar:extension/budget@0.1.0")?.func_new("remaining-fuel", |cx, _, _, out| {
        out[0] = U64(cx.get_fuel()?);
        Ok(())
    })?;
    let mut settings = linker.instance("cinnabar:extension/settings@0.1.0")?;
    settings.func_new("load", |_, _, _, out| { out[0] = ok(s("{}")); Ok(()) })?;
    settings.func_new("save", |mut cx, _, args, out| {
        let String(body) = &args[0] else { unreachable!() };
        let values: serde_json::Value = serde_json::from_str(body)?;
        assert!(values["textured"].is_boolean() && values["assist"].is_boolean());
        cx.data_mut().saved += 1;
        out[0] = unit(); Ok(())
    })?;
    let mut panel = linker.instance("cinnabar:extension/panel@0.1.0")?;
    for function in ["set-content", "update-content", "open", "close"] {
        panel.func_new(function, move |mut cx, _, args, out| {
            match function {
                "set-content" => {
                    let String(json) = &args[0] else { unreachable!() };
                    let panel: serde_json::Value = serde_json::from_str(json)?;
                    let document = panel["surface"]["document"].as_str().unwrap();
                    let parsed: serde_json::Value = serde_json::from_str(document)?;
                    assert!(parsed["menu"]["controls"].is_array());
                    cx.data_mut().pages += 1;
                }
                "open" => cx.data_mut().panel = true,
                "close" => cx.data_mut().panel = false,
                _ => {}
            }
            out[0] = unit();
            Ok(())
        })?;
    }
    let mut draw = linker.instance("cinnabar:world-draw/drawing@0.2.0")?;
    draw.func_new("capabilities", |_, _, _, out| {
        out[0] = Flags(vec!["world-draw".into(), "render-material".into()]);
        Ok(())
    })?;
    draw.func_new("geometry-status", |_, _, _, out| {
        out[0] = ok(Enum("ready".into()));
        Ok(())
    })?;
    draw.func_new("remove", |mut cx, _, args, out| {
        if let U64(id) = field(&args[0], "slot") { cx.data_mut().active.remove(id); }
        cx.data_mut().removes += 1;
        out[0] = unit();
        Ok(())
    })?;
    for function in ["create-geometry", "create"] {
        draw.func_new(function, move |mut cx, _, args, out| {
            if let Variant(kind, _) = &args[0] {
                if kind == "blocks" {
                    cx.data_mut().grids += 1;
                    if cx.data().reject_grid {
                        out[0] = Result(Err(Some(Box::new(rec([
                            ("code", Enum("invalid".into())), ("detail", s("test rejects grid")),
                        ])))));
                        return Ok(());
                    }
                } else if kind == "triangles" {
                    let Variant(_, Some(mesh)) = &args[0] else { unreachable!() };
                    let List(vertices) = field(mesh, "vertices") else { unreachable!() };
                    cx.data_mut().max_draws = cx.data().max_draws.max(vertices.len());
                    cx.data_mut().meshes += 1;
                }
            }
            cx.data_mut().next_handle += 1;
            if function == "create" { let id = cx.data().next_handle; cx.data_mut().active.insert(id); }
            out[0] = ok(rec([("owner", U64(1)), ("generation", U64(1)), ("slot", U64(cx.data().next_handle))]));
            Ok(())
        })?;
    }
    linker.instance("cinnabar:extension/terrain@0.1.0")?.func_new("read-region", move |mut cx, _, args, out| {
        cx.data_mut().calls += 1;
        let stamp = rec([("session", U64(1)), ("dimension", S32(cx.data().dimension)), ("registry", U64(1)), ("revision", U64(2))]);
        if let Option(Some(_)) = args[1] {
            out[0] = ok(Variant("unchanged".into(), Some(Box::new(stamp))));
            return Ok(());
        }
        if cx.data().calls == 1 {
            out[0] = ok(Variant("pending".into(), None));
            return Ok(());
        }
        let min = field(&args[0], "min");
        let max = field(&args[0], "max");
        let n = ["x", "y", "z"].iter().map(|axis| i(field(max, axis)) - i(field(min, axis))).product::<i32>() as usize;
        let palette = [("minecraft:air", "{}"), ("minecraft:dirt", "{}")].into_iter().enumerate().map(|(id, (name, state))| rec([
            ("runtime-id", U32(id as u32)), ("identifier", some(s(name))), ("canonical-state", some(s(state))),
            ("tags", List(vec![])), ("physics", Option(None)),
            ("collision", Variant("not-requested".into(), None)), ("selection", Variant("not-requested".into(), None)),
        ])).collect();
        let mut loaded = vec![U64(u64::MAX); n.div_ceil(64)];
        let mut states = vec![U16(0); n];
        if !stress && n > 10 {
            loaded[0] = U64(u64::MAX - 2);
            states[2] = U16(1);
            states[8] = U16(1);
        }
        let region = rec([
            ("stamp", stamp), ("chunks", List(vec![])), ("layer-count", U8(1)),
            ("loaded", List(loaded)), ("states", List(states)), ("palette", List(palette)),
            ("shape-offsets", List(vec![])), ("shape-boxes", List(vec![])), ("overrides", List(vec![])),
        ]);
        out[0] = ok(Variant("changed".into(), Some(Box::new(region))));
        Ok(())
    })?;
    let mut store = Store::new(&engine, State::default());
    store.set_fuel(1_000_000)?;
    let instance = linker.instantiate(&mut store, &component)?;
    let init = instance.get_typed_func::<(), ()>(&mut store, "init")?;
    let frame = instance.get_typed_func::<(), ()>(&mut store, "frame")?;
    init.call(&mut store, ())?;

    let mut max_fuel = 0;
    let mut advance = |store: &mut Store<State>, key, event, frames| -> Result<()> {
        for index in 0..frames {
            store.data_mut().key = if index == 0 { key } else { "" };
            store.data_mut().event = if index == 0 { event } else { "" };
            store.set_fuel(1_000_000)?;
            if let Err(error) = frame.call(&mut *store, ()) {
                anyhow::bail!("key {key}, event {event}, tick {index}: {error:#}");
            }
            max_fuel = max_fuel.max(1_000_000 - store.get_fuel()?);
        }
        Ok(())
    };
    advance(&mut store, "", "", 5)?;
    for key in ["Slash","NumpadDivide","NumpadMultiply","NumpadSubtract","ShiftRight","MouseMiddle"] {
        advance(&mut store,key,"",1)?;
    }
    ensure!(store.data().pages == 0 && store.data().active.is_empty());
    advance(&mut store, "KeyJ", "", 1)?;
    ensure!(store.data().panel);
    advance(&mut store, "", "load", 5)?;
    ensure!(store.data().panel && store.data().active.is_empty());
    advance(&mut store, "", "select-1", 1)?;
    advance(&mut store, "", "load", 165)?;
    ensure!(!store.data().panel && store.data().label.contains("passenger-wagon-2") &&
        store.data().label.contains("missing 487") && store.data().grids > 0);
    println!("Wagon: {}", store.data().label);
    let old_ids = store.data().active.clone();
    advance(&mut store, "KeyJ", "", 1)?;
    advance(&mut store, "", "select-0", 1)?;
    advance(&mut store, "", "load", 110)?;
    ensure!(store.data().active.is_disjoint(&old_ids) && store.data().label.contains("Demo arch") &&
        store.data().label.contains("missing 12"));
    println!("Demo: {}",store.data().label);
    advance(&mut store, "KeyK", "", 1)?;
    advance(&mut store, "", "assist", 1)?;
    advance(&mut store, "", "look", 1)?;
    advance(&mut store, "", "close", 1)?;
    advance(&mut store, "", "", 35)?;
    ensure!(store.data().saved == 2 && store.data().meshes > 0);
    println!("Passed. Peak fuel {max_fuel}; {} textured grids; {} panels; {} settings saves.",
        store.data().grids,store.data().pages,store.data().saved);
    Ok(())
}
