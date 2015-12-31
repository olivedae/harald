#import <util.h>
#import <unistd.h>
#import <Foundation/NSDictionary.h>
#import <IOBluetooth/IOBluetoothUserLib.h>
#import <IOBluetooth/objc/IOBluetoothDevice.h>
#import <IOBluetooth/objc/IOBluetoothDeviceInquiry.h>
#import "bluetooth_osx.h"

NSArray *bluetooth_scan(void)
{
    Scanner *scan = [Scanner new];
    [scan startSearch];
        while ([scan isBusy])
            ;
    return [scan getFoundDevices];
}

@implementation Device

@synthesize name, address;

-(void) setName:(NSString *)n andAddress:(NSString *)a
{
    name = n;
    address = a;
}
@end

@implementation Scanner

- (void) deviceInquiryDeviceFound:(IOBluetoothDeviceInquiry*)sender
                           device:(IOBluetoothDevice*)device;
{
    NSString *deviceAddress, *deviceName;

    Device *foundDevice = [Device new];

    const BluetoothDeviceAddress *addressp = [device getAddress];
    deviceAddress = [NSString stringWithFormat:@"[%02x:%02x:%02x:%02x:%02x:%02x]",
                     addressp->data[0],
                     addressp->data[1],
                     addressp->data[2],
                     addressp->data[3],
                     addressp->data[4],
                     addressp->data[5]];
    deviceName = [device name];

    if (!deviceName)
        deviceName = @"[unknown]";

    [foundDevice setName:deviceName andAddress:deviceAddress];

    [foundDevices addObject:foundDevice];
}

-(IOReturn) startSearch
{
    IOReturn status;

    [self stopSearch];

    inquiry = [IOBluetoothDeviceInquiry inquiryWithDelegate:self];

    status = [inquiry start];

    if (status == kIOReturnSuccess)
        busy = TRUE;

    return status;
}

-(void) stopSearch
{
    if (inquiry)
    {
        [inquiry stop];
        inquiry = nil;
    }
}

-(void) deviceInquiryComplete:(IOBluetoothDeviceInquiry*)sender
                        error:(IOReturn)error aborted:(BOOL)aborted
{
    busy = FALSE;
}

-(BOOL) isBusy
{
    return busy;
}

-(NSArray *) getFoundDevices
{
    return [foundDevices copy];
}
@end
