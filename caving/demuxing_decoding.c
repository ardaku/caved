/*
 * Copyright (c) 2012 Stefano Sabatini
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in
 * all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL
 * THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
 * THE SOFTWARE.
 */

/**
 * @file
 * Demuxing and decoding example.
 *
 * Show how to use the libavformat and libavcodec API to demux and
 * decode audio and video data.
 * @example demuxing_decoding.c
 */

#include<stdbool.h>

#include <libavutil/imgutils.h>
#include <libavutil/samplefmt.h>
#include <libavutil/timestamp.h>
#include <libavformat/avformat.h>

static AVFormatContext *fmt_ctx = NULL;
static AVCodecContext *video_dec_ctx = NULL, *audio_dec_ctx;
static int width, height;
static enum AVPixelFormat pix_fmt;
static AVStream *video_stream = NULL, *audio_stream = NULL;

static uint8_t *video_dst_data[4] = {NULL};
static int      video_dst_linesize[4];
static int video_dst_bufsize;

static int video_stream_idx = -1, audio_stream_idx = -1;
static AVFrame *frame = NULL;
static AVPacket pkt;
static int video_frame_count = 0;
static int audio_frame_count = 0;

static void pgm_save(
    unsigned char *buf_y,
    int pitch_y,
    unsigned char *buf_cb,
    int pitch_cb,
    unsigned char *buf_cr,
    int pitch_cr,
//    int xsize,
//    int ysize,
    void (*video_write)(void* data_y, void* data_cb, void* data_cr,
        size_t pitch_y, size_t pitch_cb, size_t pitch_cr)
) {
    video_write(buf_y, buf_cb, buf_cr, pitch_y, pitch_cb, pitch_cr);
/*    for (int i = 0; i < ysize; i++) {
        unsigned char *data_y = buf_y + (i * pitch_y);
        unsigned char *data_cb = buf_cb + ((i >> 1) * pitch_cb);
        unsigned char *data_cr = buf_cr + ((i >> 1) * pitch_cr);

        video_write(buf + (i * pitch), xsize);
    }*/

/*    FILE *f;
    int i;

    f = fopen(filename,"w");
    fprintf(f, "P5\n%d %d\n%d\n", xsize, ysize, 255);
    for (i = 0; i < ysize; i++)
        fwrite(buf + i * pitch, 1, xsize, f);
    fclose(f);*/
}

static int decode_packet(
    int *got_frame,
    int cached,
    void (*video_write)(void* data_y, void* data_cb, void* data_cr,
        size_t pitch_y, size_t pitch_cb, size_t pitch_cr),
    void (*audio_write)(void* data, size_t size)
) {
    int ret = 0;
    int decoded = pkt.size;

    *got_frame = 0;

    if (pkt.stream_index == video_stream_idx) {
        /* decode video frame */
        ret = avcodec_decode_video2(video_dec_ctx, frame, got_frame, &pkt);
        if (ret < 0) {
            fprintf(stderr, "Error decoding video frame (%s)\n", av_err2str(ret));
            return ret;
        }

        if (*got_frame) {

            if (frame->width != width || frame->height != height ||
                frame->format != pix_fmt) {
                /* To handle this change, one could call av_image_alloc again and
                 * decode the following frames into another rawvideo file. */
                fprintf(stderr, "Error: Width, height and pixel format have to be "
                        "constant in a rawvideo file, but the width, height or "
                        "pixel format of the input video changed:\n"
                        "old: width = %d, height = %d, format = %s\n"
                        "new: width = %d, height = %d, format = %s\n",
                        width, height, av_get_pix_fmt_name(pix_fmt),
                        frame->width, frame->height,
                        av_get_pix_fmt_name(frame->format));
                return -1;
            }

        printf(
            "Frame %c (%d) pts %d dts %d key_frame %d [coded_picture_number %d, display_picture_number %d]\n",
            av_get_picture_type_char(frame->pict_type),
            video_dec_ctx->frame_number,
            frame->pts,
            frame->pkt_dts,
            frame->key_frame,
            frame->coded_picture_number,
            frame->display_picture_number
        );

//            printf("video_frame%s n:%d coded_n:%d\n",
//                   cached ? "(cached)" : "",
//                   video_frame_count++, frame->coded_picture_number);

            /* write to rawvideo file */
            pgm_save(
                frame->data[0], frame->linesize[0],
                frame->data[1], frame->linesize[1],
                frame->data[2], frame->linesize[2], video_write
                /*frame->width, frame->height, "video.pgm"*/);

//            video_write(video_dst_data[0], video_dst_bufsize);
        }
    } else if (pkt.stream_index == audio_stream_idx) {
        /* decode audio frame */
        ret = avcodec_decode_audio4(audio_dec_ctx, frame, got_frame, &pkt);
        if (ret < 0) {
            fprintf(stderr, "Error decoding audio frame (%s)\n", av_err2str(ret));
            return ret;
        }
        /* Some audio decoders decode only part of the packet, and have to be
         * called again with the remainder of the packet data.
         * Sample: fate-suite/lossless-audio/luckynight-partial.shn
         * Also, some decoders might over-read the packet. */
        decoded = FFMIN(ret, pkt.size);

        if (*got_frame) {
            size_t unpadded_linesize = frame->nb_samples * av_get_bytes_per_sample(frame->format);
//            printf("audio_frame%s n:%d nb_samples:%d pts:%s\n",
//                   cached ? "(cached)" : "",
//                   audio_frame_count++, frame->nb_samples,
//                   av_ts2timestr(frame->pts, &audio_dec_ctx->time_base));

            /* Write the raw audio data samples of the first plane. This works
             * fine for packed formats (e.g. AV_SAMPLE_FMT_S16). However,
             * most audio decoders output planar audio, which uses a separate
             * plane of audio samples for each channel (e.g. AV_SAMPLE_FMT_S16P).
             * In other words, this code will write only the first audio channel
             * in these cases.
             * You should use libswresample or libavfilter to convert the frame
             * to packed data. */
            audio_write(frame->extended_data[0], unpadded_linesize);
        }
    }

    return decoded;
}

static int open_codec_context(int *stream_idx,
                              AVCodecContext **dec_ctx, AVFormatContext *fmt_ctx, enum AVMediaType type, AVRational* fps)
{
    int ret, stream_index;
    AVStream *st;
    AVCodec *dec = NULL;
    AVDictionary *opts = NULL;

    ret = av_find_best_stream(fmt_ctx, type, -1, -1, NULL, 0);
    if (ret < 0) {
        fprintf(stderr, "Could not find %s stream in input file\n",
                av_get_media_type_string(type));
        return ret;
    } else {
        stream_index = ret;
        st = fmt_ctx->streams[stream_index];

        /* find decoder for the stream */
        dec = avcodec_find_decoder(st->codecpar->codec_id);
        if (!dec) {
            fprintf(stderr, "Failed to find %s codec\n",
                    av_get_media_type_string(type));
            return AVERROR(EINVAL);
        }

        /* Allocate a codec context for the decoder */
        *dec_ctx = avcodec_alloc_context3(dec);
        if (!*dec_ctx) {
            fprintf(stderr, "Failed to allocate the %s codec context\n",
                    av_get_media_type_string(type));
            return AVERROR(ENOMEM);
        }

        /* Copy codec parameters from input stream to output codec context */
        if ((ret = avcodec_parameters_to_context(*dec_ctx, st->codecpar)) < 0) {
            fprintf(stderr, "Failed to copy %s codec parameters to decoder context\n",
                    av_get_media_type_string(type));
            return ret;
        }

        /* Init the decoders, with or without reference counting */
        av_dict_set(&opts, "refcounted_frames", "0", 0);
        if ((ret = avcodec_open2(*dec_ctx, dec, &opts)) < 0) {
            fprintf(stderr, "Failed to open %s codec\n",
                    av_get_media_type_string(type));
            return ret;
        }
        *stream_idx = stream_index;

        if (type == AVMEDIA_TYPE_VIDEO) {
            *fps = st->avg_frame_rate;
        }
    }

    return 0;
}

// Read file input_file, and write raw audio and video.
void caving_decode_new(
    const char* src_filename,
    int* video_width,
    int* video_height,
    enum AVPixelFormat* pixel_format,
    int* audio_channels,
    int* audio_samplerate,
    enum AVSampleFormat* audio_sampleformat,
    AVRational* video_fps
) {
    // open input file, and allocate format context
    if (avformat_open_input(&fmt_ctx, src_filename, NULL, NULL) < 0) {
        fprintf(stderr, "Could not open source file %s\n", src_filename);
        exit(1);
    }

    // retrieve stream information
    if (avformat_find_stream_info(fmt_ctx, NULL) < 0) {
        fprintf(stderr, "Could not find stream information\n");
        exit(1);
    }

    // Open video stream, if it exists.
    if (open_codec_context(&video_stream_idx, &video_dec_ctx, fmt_ctx, AVMEDIA_TYPE_VIDEO, video_fps) >= 0) {
        // Set parameters
        *video_width = video_dec_ctx->width;
        *video_height = video_dec_ctx->height;
        *pixel_format = video_dec_ctx->pix_fmt;

        // Set Stream
        video_stream = fmt_ctx->streams[video_stream_idx];

        /* allocate image where the decoded image will be put */
        width = *video_width;
        height = *video_height;
        pix_fmt = *pixel_format;
        video_dst_bufsize = av_image_alloc(video_dst_data, video_dst_linesize,
                             width, height, pix_fmt, 1);
        if (video_dst_bufsize < 0) {
            fprintf(stderr, "Could not allocate raw video buffer\n");
            exit(1);
        }
    }

    // Open audio stream, if it exists.
    if (open_codec_context(&audio_stream_idx, &audio_dec_ctx, fmt_ctx, AVMEDIA_TYPE_AUDIO, video_fps) >= 0) {
        // Set parameters
        *audio_channels = audio_dec_ctx->channels;
        *audio_samplerate = audio_dec_ctx->sample_rate;
        *audio_sampleformat = audio_dec_ctx->sample_fmt;

        if (av_sample_fmt_is_planar(*audio_sampleformat)) {
            printf("CAVING > WARNING: Audio stream will be played mono.\n");
            *audio_sampleformat = av_get_packed_sample_fmt(*audio_sampleformat);
            *audio_channels = 1;
        }

        // Set Stream
        audio_stream = fmt_ctx->streams[audio_stream_idx];
    }

    /* dump input information to stderr */
    // av_dump_format(fmt_ctx, 0, src_filename, 0);

    if (!audio_stream && !video_stream) {
        fprintf(stderr, "Could not find audio or video stream in the input, aborting\n");
        exit(1);
    }

    frame = av_frame_alloc();
    if (!frame) {
        fprintf(stderr, "Could not allocate frame\n");
        exit(1);
    }

    /* initialize packet, set data to NULL, let the demuxer fill it */
    av_init_packet(&pkt);
    pkt.data = NULL;
    pkt.size = 0;
}

bool caving_decode_run(
    void (*video_write)(void* data_y, void* data_cb, void* data_cr,
        size_t pitch_y, size_t pitch_cb, size_t pitch_cr),
    void (*audio_write)(void* data, size_t size)
) {
    int got_frame = 0;

    // read frames from the file
    if (av_read_frame(fmt_ctx, &pkt) >= 0) {
        AVPacket orig_pkt = pkt;
        do {
            int ret = decode_packet(&got_frame, 0, video_write, audio_write);
            if (ret < 0)
                break;
            pkt.data += ret;
            pkt.size -= ret;
        } while (pkt.size > 0);
        av_packet_unref(&orig_pkt);
//        if(got_frame) return false;
//        else return caving_decode_run(video_write, audio_write);
        return false;
    } else {
        // flush cached frames
        pkt.data = NULL;
        pkt.size = 0;
        do {
            decode_packet(&got_frame, 1, video_write, audio_write);
        } while (got_frame);

        avcodec_free_context(&video_dec_ctx);
        avcodec_free_context(&audio_dec_ctx);
        avformat_close_input(&fmt_ctx);
        av_frame_free(&frame);
        av_free(video_dst_data[0]);

        return true;
    }
}
