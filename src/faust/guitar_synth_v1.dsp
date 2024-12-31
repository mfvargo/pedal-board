import("stdfaust.lib");

// basic functions

in_left = (_,_ : _,!); // left in -  mono input

limit_value(x,minval,maxval) = (x, minval : select2(x < minval)) : (_, maxval : select2(x > maxval));   // limit value


// Define Control Sliders
att = hslider("h:[0]ADSR/[0]Attack",0.01,0.01,4,0.01);
dec = hslider("h:[0]ADSR/[1]Decay",0.24,0,1,0.01);
sus = hslider("h:[0]ADSR/[2]Sustain",0.3,0,10,0.01);
rel = hslider("h:[0]ADSR/[3]Release",0.25,0,10,0.01);
trigger_thresh = hslider("h:[0]ADSR/[4]Threshold",.4,0.05,1,0.01);
env_out = hbargraph("h:[1]Triggered ADSR Output/[0]Envelope",0,10000);

// pitch shift effect - voice 2
window = hslider("h:[2]Pitchshifter/[0]Window (samples)", 2048, 50, 10000, 1);
xfade = hslider("h:[2]Pitchshifter/[1]xfade (samples)", 256, 1, 10000, 1);
shift = hslider("h:[2]Pitchshifter/[2]shift (semitones) ", -12, -12, +12, 0.1);
				
pitchshifter = _ : ef.transpose(window, xfade, shift);

// Filter Params
filter_freq = hslider("h:[3]Filter/[0]Filter Freq",2500,50,10000,0.01) : si.smoo;
filter_q = hslider("h:[3]Filter/[1]Filter Q",25,0.2,50,0.01) : si.smoo;
gain = hslider("h:[3]Filter/[2]Gain",0.85,0,1,0.01);

// generate a gated ADSR waveform - triggered by guitar
ggate = fi.lowpass(4,2000) : (25*an.an.rms_envelope_rect(.015) > trigger_thresh) : en.adsre(att,sus,dec,rel)*gain;

// freq modulation function - final signal to control filter
freq_mod(x) = limit_value((x : ggate : filter_freq,_ : *),40,10000) : env_out;

// define synth function - Moog style filter
synth(x) =  _ <: ggate , (x*ggate : ve.moog_vcf_2b(filter_q/40,freq_mod(x))) : !,_;    // volume gate + Moog Ladder Filter
//synth(x) =  _ <: ggate , (ve.moog_vcf_2b(filter_q/40,freq_mod(x))) : !,_;    // Moog Ladder Filter
//synth(x) =  _ <: ggate , (x*ggate : ve.oberheimLPF(freq_mod(x)/3300, filter_q/40)) : !,_;  // CAUTION - can go unstable (LOUD) if Filter Freq slider set above 5-6000!!!
//synth(x) =  _ <: ggate , (x*ggate : fi.resonbp(freq_mod(x)/2, filter_q/5,1)) : !,_;

// delay effect
delay_level    = hslider("h:[4]Delay/[0]Delay Level[name:Gain]", -24, -80, 6, 1) : ba.db2linear : si.smooth(0.999);
feedback = hslider("h:[4]Delay/[0]Feedback[name:feedback]", 0.2, 0, 1.2, 0.01) : si.smooth(0.999);
delay = hslider("h:[4]Delay/[0]Delay Time (samples)",18000, 0, 80000, 1);

delay_effect = _ <: _ + delay_level*(_ : + ~ (feedback * (_ ,delay : (@ : fi.dcblocker)) ) ) ;

// Output Mixer 
dry_level = hslider("h:[5]Out Mix/[0]dry level", 0, 0,.5 ,0.1);
fundamental_level = hslider("h:[5]Out Mix/[1]Fundamental ", 0.7, 0, 1, .01);
pitchshift_level = hslider("h:[5]Out Mix/[2]Pitchshift Level ", 0.7, 0, 1, .01);

process = in_left <: (_*fundamental_level, pitchshift_level*pitchshifter : + <: synth : delay_effect), _* dry_level : + <: _,_;