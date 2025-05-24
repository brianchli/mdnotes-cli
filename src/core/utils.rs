#[macro_export]
macro_rules! write_coloured {

    // bold write
    ($stream: ident, bold, $($arg: tt)+) => {
        $stream.set_color(ColorSpec::new().set_bold(true))?;
        write!($stream, $($arg)+)?;
        $stream.reset()?;
    };

    // write coloured
    ($stream: ident, colour=$colour: expr, $($arg: tt)+) => {
        $stream.set_color(ColorSpec::new().set_fg(Some($colour)))?;
        write!($stream, $($arg)+)?;
        $stream.reset()?;
    };

    // write coloured and bolded
    ($stream: ident, bold_colour=$colour: expr, $($arg: tt)+) => {
        $stream.set_color(ColorSpec::new().set_bold(true).set_fg(Some($colour)))?;
        write!($stream, $($arg)+)?;
        $stream.reset()?;
    };

}

#[macro_export]
macro_rules! write_colouredln {

    // bold write
    ($stream: ident, bold, $($arg: tt)+) => {
        $stream.set_color(ColorSpec::new().set_bold(true))?;
        writeln!($stream, $($arg)+)?;
        $stream.reset()?;
    };

    // write coloured
    ($stream: ident, colour=$colour: expr, $($arg: tt)+) => {
        $stream.set_color(ColorSpec::new().set_fg(Some($colour)))?;
        writeln!($stream, $($arg)+)?;
        $stream.reset()?;
    };

    // write coloured and bolded
    ($stream: ident, bold_colour=$colour: expr, $($arg: tt)+) => {
        $stream.set_color(ColorSpec::new().set_bold(true).set_fg(Some($colour)))?;
        writeln!($stream, $($arg)+)?;
        $stream.reset()?;
    };

}
